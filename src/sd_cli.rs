use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::ModelConfig;
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct GenerateArgs {
    pub sd_cli_path: PathBuf,

    // Model paths
    pub model: Option<PathBuf>,
    pub clip_l: Option<PathBuf>,
    pub clip_g: Option<PathBuf>,
    pub t5xxl: Option<PathBuf>,
    pub diffusion_model: Option<PathBuf>,
    pub vae: Option<PathBuf>,

    // Generation parameters
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub width: u32,
    pub height: u32,
    pub steps: u32,
    pub cfg_scale: f32,
    pub guidance: Option<f32>,
    pub seed: i64,
    pub sampling_method: String,
    pub scheduler: String,
    pub batch_count: u32,
    pub output: PathBuf,
}

impl GenerateArgs {
    pub fn from_model_config(sd_cli_path: &Path, model_config: &ModelConfig) -> Self {
        Self {
            sd_cli_path: sd_cli_path.to_path_buf(),
            model: model_config.model.clone(),
            clip_l: model_config.clip_l.clone(),
            clip_g: model_config.clip_g.clone(),
            t5xxl: model_config.t5xxl.clone(),
            diffusion_model: model_config.diffusion_model.clone(),
            vae: model_config.vae.clone(),
            prompt: String::new(),
            negative_prompt: model_config.negative_prompt.clone(),
            width: model_config.width.unwrap_or(512),
            height: model_config.height.unwrap_or(512),
            steps: model_config.steps.unwrap_or(20),
            cfg_scale: model_config.cfg_scale.unwrap_or(7.0),
            guidance: model_config.guidance,
            seed: model_config.seed.unwrap_or(-1),
            sampling_method: model_config
                .sampling_method
                .clone()
                .unwrap_or_else(|| "euler_a".into()),
            scheduler: model_config
                .scheduler
                .clone()
                .unwrap_or_else(|| "discrete".into()),
            batch_count: model_config.batch_count.unwrap_or(1),
            output: PathBuf::from("output.png"),
        }
    }

    pub fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        // Model paths
        if let Some(m) = &self.model {
            args.extend(["-m".into(), m.to_string_lossy().into()]);
        }
        if let Some(p) = &self.clip_l {
            args.extend(["--clip_l".into(), p.to_string_lossy().into()]);
        }
        if let Some(p) = &self.clip_g {
            args.extend(["--clip_g".into(), p.to_string_lossy().into()]);
        }
        if let Some(p) = &self.t5xxl {
            args.extend(["--t5xxl".into(), p.to_string_lossy().into()]);
        }
        if let Some(p) = &self.diffusion_model {
            args.extend(["--diffusion-model".into(), p.to_string_lossy().into()]);
        }
        if let Some(p) = &self.vae {
            args.extend(["--vae".into(), p.to_string_lossy().into()]);
        }

        // Generation parameters
        args.extend(["-p".into(), self.prompt.clone()]);

        if let Some(neg) = &self.negative_prompt
            && !neg.is_empty()
        {
            args.extend(["-n".into(), neg.clone()]);
        }

        args.extend(["-W".into(), self.width.to_string()]);
        args.extend(["-H".into(), self.height.to_string()]);
        args.extend(["--steps".into(), self.steps.to_string()]);
        args.extend(["--cfg-scale".into(), self.cfg_scale.to_string()]);

        if let Some(g) = self.guidance {
            args.extend(["--guidance".into(), g.to_string()]);
        }

        args.extend(["-s".into(), self.seed.to_string()]);
        args.extend(["--sampling-method".into(), self.sampling_method.clone()]);
        args.extend(["--scheduler".into(), self.scheduler.clone()]);

        if self.batch_count > 1 {
            args.extend(["-b".into(), self.batch_count.to_string()]);
        }

        args.extend(["-o".into(), self.output.to_string_lossy().into()]);

        args
    }

    pub fn validate(&self) -> Result<()> {
        if !self.sd_cli_path.exists() {
            return Err(Error::SdCliNotFound(self.sd_cli_path.clone()));
        }
        Ok(())
    }

    pub fn run(&self) -> Result<PathBuf> {
        self.validate()?;

        let args = self.to_args();
        let output = Command::new(&self.sd_cli_path).args(&args).output()?;

        if !output.status.success() {
            let exit_code = output.status.code().ok_or(Error::SdCliKilled)?;
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::SdCliFailed { exit_code, stderr });
        }

        Ok(self.output.clone())
    }
}

#[cfg(test)]
/// Run sd-cli with a custom command runner (for testing).
fn run_with_command(args: &GenerateArgs, mut cmd: Command) -> Result<PathBuf> {
    let cli_args = args.to_args();
    let output = cmd.args(&cli_args).output()?;

    if !output.status.success() {
        let exit_code = output.status.code().ok_or(Error::SdCliKilled)?;
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(Error::SdCliFailed { exit_code, stderr });
    }

    Ok(args.output.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;

    fn sample_config() -> AppConfig {
        AppConfig::parse(
            r#"
sd_cli_path = "/usr/bin/sd-cli"

[models.sd15]
model = "/models/sd15.safetensors"
width = 512
height = 512
steps = 20
cfg_scale = 7.0
sampling_method = "euler_a"
scheduler = "discrete"

[models.flux]
diffusion_model = "/models/flux-dev.gguf"
clip_l = "/models/clip_l.safetensors"
t5xxl = "/models/t5xxl.safetensors"
vae = "/models/ae.safetensors"
width = 1024
height = 1024
steps = 20
guidance = 3.5
sampling_method = "euler"
scheduler = "simple"
"#,
        )
        .expect("test config should parse")
    }

    #[test]
    fn from_model_config_single_file() {
        let config = sample_config();
        let model = config.resolve_model("sd15").expect("sd15 exists");
        let args = GenerateArgs::from_model_config(&config.sd_cli_path, model);

        assert_eq!(args.width, 512);
        assert_eq!(args.height, 512);
        assert_eq!(args.steps, 20);
        assert_eq!(args.cfg_scale, 7.0);
        assert_eq!(args.sampling_method, "euler_a");
        assert!(args.model.is_some());
        assert!(args.diffusion_model.is_none());
    }

    #[test]
    fn from_model_config_component_paths() {
        let config = sample_config();
        let model = config.resolve_model("flux").expect("flux exists");
        let args = GenerateArgs::from_model_config(&config.sd_cli_path, model);

        assert_eq!(args.width, 1024);
        assert_eq!(args.guidance, Some(3.5));
        assert!(args.diffusion_model.is_some());
        assert!(args.clip_l.is_some());
        assert!(args.t5xxl.is_some());
        assert!(args.vae.is_some());
        assert!(args.model.is_none());
    }

    #[test]
    fn to_args_single_file_model() {
        let config = sample_config();
        let model = config.resolve_model("sd15").expect("sd15 exists");
        let mut ga = GenerateArgs::from_model_config(&config.sd_cli_path, model);
        ga.prompt = "a cat".into();
        ga.output = PathBuf::from("out.png");

        let args = ga.to_args();
        assert!(args.contains(&"-m".to_string()));
        assert!(args.contains(&"/models/sd15.safetensors".to_string()));
        assert!(args.contains(&"-p".to_string()));
        assert!(args.contains(&"a cat".to_string()));
        assert!(args.contains(&"-o".to_string()));
        assert!(args.contains(&"out.png".to_string()));
        // No --guidance for SD1.5
        assert!(!args.contains(&"--guidance".to_string()));
    }

    #[test]
    fn to_args_component_model() {
        let config = sample_config();
        let model = config.resolve_model("flux").expect("flux exists");
        let mut ga = GenerateArgs::from_model_config(&config.sd_cli_path, model);
        ga.prompt = "a dog".into();

        let args = ga.to_args();
        assert!(args.contains(&"--diffusion-model".to_string()));
        assert!(args.contains(&"--clip_l".to_string()));
        assert!(args.contains(&"--t5xxl".to_string()));
        assert!(args.contains(&"--vae".to_string()));
        assert!(args.contains(&"--guidance".to_string()));
        assert!(args.contains(&"3.5".to_string()));
        // No -m for component model
        assert!(!args.contains(&"-m".to_string()));
    }

    #[test]
    fn to_args_negative_prompt() {
        let config = sample_config();
        let model = config.resolve_model("sd15").expect("sd15 exists");
        let mut ga = GenerateArgs::from_model_config(&config.sd_cli_path, model);
        ga.prompt = "a cat".into();
        ga.negative_prompt = Some("ugly, blurry".into());

        let args = ga.to_args();
        assert!(args.contains(&"-n".to_string()));
        assert!(args.contains(&"ugly, blurry".to_string()));
    }

    #[test]
    fn to_args_batch_count_omitted_when_one() {
        let config = sample_config();
        let model = config.resolve_model("sd15").expect("sd15 exists");
        let mut ga = GenerateArgs::from_model_config(&config.sd_cli_path, model);
        ga.prompt = "test".into();
        ga.batch_count = 1;

        let args = ga.to_args();
        assert!(!args.contains(&"-b".to_string()));
    }

    #[test]
    fn to_args_batch_count_included_when_greater_than_one() {
        let config = sample_config();
        let model = config.resolve_model("sd15").expect("sd15 exists");
        let mut ga = GenerateArgs::from_model_config(&config.sd_cli_path, model);
        ga.prompt = "test".into();
        ga.batch_count = 4;

        let args = ga.to_args();
        let idx = args.iter().position(|a| a == "-b").expect("-b present");
        assert_eq!(args[idx + 1], "4");
    }

    #[test]
    fn defaults_when_model_config_is_sparse() {
        let config = AppConfig::parse(
            r#"
[models.minimal]
model = "/models/m.safetensors"
"#,
        )
        .expect("should parse");
        let model = config.resolve_model("minimal").expect("exists");
        let args = GenerateArgs::from_model_config(&config.sd_cli_path, model);

        assert_eq!(args.width, 512);
        assert_eq!(args.height, 512);
        assert_eq!(args.steps, 20);
        assert_eq!(args.cfg_scale, 7.0);
        assert_eq!(args.seed, -1);
        assert_eq!(args.sampling_method, "euler_a");
        assert_eq!(args.scheduler, "discrete");
        assert_eq!(args.batch_count, 1);
    }

    #[test]
    fn validate_missing_sd_cli() {
        let config = sample_config();
        let model = config.resolve_model("sd15").expect("sd15 exists");
        let mut args = GenerateArgs::from_model_config(&config.sd_cli_path, model);
        args.sd_cli_path = PathBuf::from("/nonexistent/sd-cli");
        args.prompt = "test".into();

        let err = args.validate().unwrap_err();
        assert!(err.to_string().contains("sd-cli binary not found"));
    }

    #[test]
    fn run_with_command_failure() {
        let config = sample_config();
        let model = config.resolve_model("sd15").expect("sd15 exists");
        let mut args = GenerateArgs::from_model_config(&config.sd_cli_path, model);
        args.prompt = "test".into();

        // Use `false` command which always exits with code 1
        let cmd = Command::new("false");
        let err = run_with_command(&args, cmd).unwrap_err();
        assert!(err.to_string().contains("sd-cli failed"));
    }

    #[test]
    fn run_with_command_success() {
        let config = sample_config();
        let model = config.resolve_model("sd15").expect("sd15 exists");
        let mut args = GenerateArgs::from_model_config(&config.sd_cli_path, model);
        args.prompt = "test".into();
        args.output = PathBuf::from("/tmp/test.png");

        // Use `true` command which always exits with code 0
        let cmd = Command::new("true");
        let result = run_with_command(&args, cmd).expect("should succeed");
        assert_eq!(result, PathBuf::from("/tmp/test.png"));
    }
}
