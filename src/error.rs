use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("config file not found: {0}")]
    ConfigNotFound(PathBuf),

    #[error("failed to parse config: {0}")]
    InvalidConfig(String),

    #[error("model not found: {name}")]
    ModelNotFound { name: String },

    #[error("no --model specified and no default_model in config")]
    NoDefaultModel,

    #[error("sd-cli binary not found: {0}")]
    SdCliNotFound(PathBuf),

    #[error("model file not found: {0}")]
    ModelFileNotFound(PathBuf),

    #[error("sd-cli failed (exit code {exit_code}): {stderr}")]
    SdCliFailed { exit_code: i32, stderr: String },

    #[error("sd-cli was killed by signal")]
    SdCliKilled,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    TomlParse(#[from] toml::de::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_not_found_display() {
        let err = Error::ConfigNotFound(PathBuf::from("/foo/config.toml"));
        assert_eq!(err.to_string(), "config file not found: /foo/config.toml");
    }

    #[test]
    fn model_not_found_display() {
        let err = Error::ModelNotFound {
            name: "sd15".into(),
        };
        assert_eq!(err.to_string(), "model not found: sd15");
    }

    #[test]
    fn sd_cli_not_found_display() {
        let err = Error::SdCliNotFound(PathBuf::from("/usr/bin/sd-cli"));
        assert_eq!(err.to_string(), "sd-cli binary not found: /usr/bin/sd-cli");
    }

    #[test]
    fn sd_cli_failed_display() {
        let err = Error::SdCliFailed {
            exit_code: 1,
            stderr: "boom".into(),
        };
        assert_eq!(err.to_string(), "sd-cli failed (exit code 1): boom");
    }

    #[test]
    fn sd_cli_killed_display() {
        let err = Error::SdCliKilled;
        assert_eq!(err.to_string(), "sd-cli was killed by signal");
    }

    #[test]
    fn invalid_config_display() {
        let err = Error::InvalidConfig("bad toml".into());
        assert_eq!(err.to_string(), "failed to parse config: bad toml");
    }

    #[test]
    fn model_file_not_found_display() {
        let err = Error::ModelFileNotFound(PathBuf::from("/models/v1.safetensors"));
        assert_eq!(
            err.to_string(),
            "model file not found: /models/v1.safetensors"
        );
    }

    #[test]
    fn io_error_transparent() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
        let err = Error::Io(io_err);
        assert!(err.to_string().contains("gone"));
    }
}
