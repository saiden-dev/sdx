use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{Error, Result};

#[derive(Debug, Deserialize, PartialEq)]
pub struct AppConfig {
    pub sd_cli_path: Option<PathBuf>,
    pub default_model: Option<String>,
    #[serde(default)]
    pub models: HashMap<String, ModelConfig>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ModelConfig {
    // Single-file model
    pub model: Option<PathBuf>,

    // Component model paths (Flux, SDXL, etc.)
    pub clip_l: Option<PathBuf>,
    pub clip_g: Option<PathBuf>,
    pub t5xxl: Option<PathBuf>,
    pub diffusion_model: Option<PathBuf>,
    pub vae: Option<PathBuf>,

    // Default generation parameters
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub steps: Option<u32>,
    pub cfg_scale: Option<f32>,
    pub guidance: Option<f32>,
    pub sampling_method: Option<String>,
    pub scheduler: Option<String>,
    pub seed: Option<i64>,
    pub batch_count: Option<u32>,
    pub negative_prompt: Option<String>,
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(Error::ConfigNotFound(path.to_path_buf()));
        }
        let contents = std::fs::read_to_string(path)?;
        Self::parse(&contents)
    }

    pub fn parse(toml_str: &str) -> Result<Self> {
        let config: AppConfig = toml::from_str(toml_str)?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        for (name, model) in &self.models {
            if model.model.is_none() && model.diffusion_model.is_none() {
                return Err(Error::InvalidConfig(format!(
                    "model '{name}' must have either 'model' or \
                     'diffusion_model' path"
                )));
            }
        }
        Ok(())
    }

    /// Return the configured sd_cli_path, or extract the embedded binary.
    pub fn resolve_sd_cli_path(&self) -> Result<PathBuf> {
        match &self.sd_cli_path {
            Some(path) => Ok(path.clone()),
            None => crate::embedded::extract(),
        }
    }

    pub fn resolve_model(&self, name: &str) -> Result<&ModelConfig> {
        self.models
            .get(name)
            .ok_or_else(|| Error::ModelNotFound { name: name.into() })
    }

    pub fn resolve_model_name<'a>(&'a self, name: Option<&'a str>) -> Result<&'a str> {
        match name {
            Some(n) => Ok(n),
            None => self.default_model.as_deref().ok_or(Error::NoDefaultModel),
        }
    }

    pub fn default_config_path() -> PathBuf {
        dirs_or_home().join("sdx").join("config.toml")
    }
}

fn dirs_or_home() -> PathBuf {
    std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| PathBuf::from(h).join(".config"))
                .unwrap_or_else(|_| PathBuf::from(".config"))
        })
}

impl ModelConfig {
    pub fn has_component_paths(&self) -> bool {
        self.diffusion_model.is_some()
    }

    pub fn model_paths(&self) -> Vec<(&str, &Path)> {
        let mut paths = Vec::new();
        if let Some(p) = &self.model {
            paths.push(("model", p.as_path()));
        }
        if let Some(p) = &self.clip_l {
            paths.push(("clip_l", p.as_path()));
        }
        if let Some(p) = &self.clip_g {
            paths.push(("clip_g", p.as_path()));
        }
        if let Some(p) = &self.t5xxl {
            paths.push(("t5xxl", p.as_path()));
        }
        if let Some(p) = &self.diffusion_model {
            paths.push(("diffusion_model", p.as_path()));
        }
        if let Some(p) = &self.vae {
            paths.push(("vae", p.as_path()));
        }
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_CONFIG: &str = r#"
[models.sd15]
model = "/models/sd15.safetensors"
"#;

    const FULL_CONFIG: &str = r#"
sd_cli_path = "/usr/local/bin/sd-cli"

[models.sd15]
model = "/models/sd15.safetensors"
width = 512
height = 512
steps = 20
cfg_scale = 7.0
sampling_method = "euler_a"
scheduler = "discrete"

[models.flux]
diffusion_model = "/models/flux-dev-q8_0.gguf"
clip_l = "/models/clip_l.safetensors"
t5xxl = "/models/t5xxl_fp16.safetensors"
vae = "/models/ae.safetensors"
width = 1024
height = 1024
steps = 20
guidance = 3.5
sampling_method = "euler"
scheduler = "simple"
"#;

    #[test]
    fn parse_minimal_config() {
        let config = AppConfig::parse(MINIMAL_CONFIG).expect("should parse");
        assert!(config.sd_cli_path.is_none());
        assert!(config.models.contains_key("sd15"));
    }

    #[test]
    fn parse_full_config() {
        let config = AppConfig::parse(FULL_CONFIG).expect("should parse");
        assert_eq!(
            config.sd_cli_path,
            Some(PathBuf::from("/usr/local/bin/sd-cli"))
        );
        assert_eq!(config.models.len(), 2);

        let sd15 = config.resolve_model("sd15").expect("sd15 exists");
        assert_eq!(
            sd15.model.as_deref(),
            Some(Path::new("/models/sd15.safetensors"))
        );
        assert_eq!(sd15.width, Some(512));
        assert_eq!(sd15.steps, Some(20));

        let flux = config.resolve_model("flux").expect("flux exists");
        assert!(flux.has_component_paths());
        assert_eq!(flux.width, Some(1024));
        assert_eq!(flux.guidance, Some(3.5));
    }

    #[test]
    fn resolve_missing_model_returns_error() {
        let config = AppConfig::parse(MINIMAL_CONFIG).expect("should parse");
        let err = config.resolve_model("nonexistent").unwrap_err();
        assert!(err.to_string().contains("nonexistent"));
    }

    #[test]
    fn validate_rejects_model_without_paths() {
        let bad = r#"
[models.bad]
width = 512
"#;
        let err = AppConfig::parse(bad).unwrap_err();
        assert!(err.to_string().contains("must have either"));
    }

    #[test]
    fn model_paths_returns_all_set_paths() {
        let config = AppConfig::parse(FULL_CONFIG).expect("should parse");
        let flux = config.resolve_model("flux").expect("flux exists");
        let paths = flux.model_paths();
        assert_eq!(paths.len(), 4);
        let labels: Vec<&str> = paths.iter().map(|(l, _)| *l).collect();
        assert!(labels.contains(&"diffusion_model"));
        assert!(labels.contains(&"clip_l"));
        assert!(labels.contains(&"t5xxl"));
        assert!(labels.contains(&"vae"));
    }

    #[test]
    fn default_config_path_is_under_config_dir() {
        let path = AppConfig::default_config_path();
        assert!(path.to_string_lossy().contains("sdx"));
        assert!(path.to_string_lossy().ends_with("config.toml"));
    }

    #[test]
    fn parse_empty_models_section() {
        let config = AppConfig::parse("").expect("empty is valid");
        assert!(config.models.is_empty());
    }

    #[test]
    fn resolve_model_name_explicit() {
        let config = AppConfig::parse(MINIMAL_CONFIG).expect("should parse");
        assert_eq!(config.resolve_model_name(Some("sd15")).unwrap(), "sd15");
    }

    #[test]
    fn resolve_model_name_uses_default() {
        let config = AppConfig::parse(
            r#"
default_model = "sd15"

[models.sd15]
model = "/models/sd15.safetensors"
"#,
        )
        .expect("should parse");
        assert_eq!(config.resolve_model_name(None).unwrap(), "sd15");
    }

    #[test]
    fn resolve_model_name_no_default_returns_error() {
        let config = AppConfig::parse(MINIMAL_CONFIG).expect("should parse");
        let err = config.resolve_model_name(None).unwrap_err();
        assert!(err.to_string().contains("no --model"));
    }

    #[test]
    fn model_config_single_file_not_component() {
        let config = AppConfig::parse(MINIMAL_CONFIG).expect("should parse");
        let sd15 = config.resolve_model("sd15").expect("sd15 exists");
        assert!(!sd15.has_component_paths());
    }
}
