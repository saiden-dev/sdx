use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "sdx", version, about = "Friendly wrapper for sd-cli")]
pub struct Cli {
    /// Path to config file
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate an image from a text prompt
    Generate(GenerateCmd),
    /// Start an OpenAI-compatible HTTP API server
    Serve(ServeCmd),
    /// List configured models
    Models,
}

#[derive(Debug, Parser)]
pub struct GenerateCmd {
    /// Model name from config (uses default_model if omitted)
    #[arg(long)]
    pub model: Option<String>,

    /// Text prompt
    #[arg(short, long)]
    pub prompt: String,

    /// Negative prompt
    #[arg(short, long)]
    pub negative_prompt: Option<String>,

    /// Image width in pixels
    #[arg(short = 'W', long)]
    pub width: Option<u32>,

    /// Image height in pixels
    #[arg(short = 'H', long)]
    pub height: Option<u32>,

    /// Number of sampling steps
    #[arg(long)]
    pub steps: Option<u32>,

    /// CFG scale
    #[arg(long)]
    pub cfg_scale: Option<f32>,

    /// Distilled guidance (Flux/SD3)
    #[arg(long)]
    pub guidance: Option<f32>,

    /// RNG seed (negative for random)
    #[arg(short, long, allow_negative_numbers = true)]
    pub seed: Option<i64>,

    /// Sampling method
    #[arg(long)]
    pub sampler: Option<String>,

    /// Scheduler
    #[arg(long)]
    pub scheduler: Option<String>,

    /// Output file path
    #[arg(short, long, default_value = "output.png")]
    pub output: PathBuf,

    /// Number of images to generate
    #[arg(short, long)]
    pub batch_count: Option<u32>,
}

#[derive(Debug, Parser)]
pub struct ServeCmd {
    /// Host address to bind
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Port to listen on
    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_parses_generate() {
        let cli = Cli::parse_from(["sdx", "generate", "--model", "sd15", "-p", "a cat"]);
        match cli.command {
            Commands::Generate(cmd) => {
                assert_eq!(cmd.model.as_deref(), Some("sd15"));
                assert_eq!(cmd.prompt, "a cat");
                assert_eq!(cmd.output, PathBuf::from("output.png"));
            }
            _ => panic!("expected Generate"),
        }
    }

    #[test]
    fn cli_parses_generate_without_model() {
        let cli = Cli::parse_from(["sdx", "generate", "-p", "a cat"]);
        match cli.command {
            Commands::Generate(cmd) => {
                assert_eq!(cmd.model, None);
                assert_eq!(cmd.prompt, "a cat");
            }
            _ => panic!("expected Generate"),
        }
    }

    #[test]
    fn cli_parses_generate_all_options() {
        let cli = Cli::parse_from([
            "sdx",
            "generate",
            "--model",
            "flux",
            "-p",
            "a dog",
            "-n",
            "ugly",
            "-W",
            "1024",
            "-H",
            "1024",
            "--steps",
            "30",
            "--cfg-scale",
            "5.0",
            "--guidance",
            "3.5",
            "-s",
            "42",
            "--sampler",
            "euler",
            "--scheduler",
            "simple",
            "-o",
            "my.png",
            "-b",
            "2",
        ]);
        match cli.command {
            Commands::Generate(cmd) => {
                assert_eq!(cmd.width, Some(1024));
                assert_eq!(cmd.height, Some(1024));
                assert_eq!(cmd.steps, Some(30));
                assert_eq!(cmd.guidance, Some(3.5));
                assert_eq!(cmd.seed, Some(42));
                assert_eq!(cmd.batch_count, Some(2));
                assert_eq!(cmd.negative_prompt.as_deref(), Some("ugly"));
            }
            _ => panic!("expected Generate"),
        }
    }

    #[test]
    fn cli_parses_serve() {
        let cli = Cli::parse_from(["sdx", "serve", "--host", "0.0.0.0", "--port", "9090"]);
        match cli.command {
            Commands::Serve(cmd) => {
                assert_eq!(cmd.host, "0.0.0.0");
                assert_eq!(cmd.port, 9090);
            }
            _ => panic!("expected Serve"),
        }
    }

    #[test]
    fn cli_parses_serve_defaults() {
        let cli = Cli::parse_from(["sdx", "serve"]);
        match cli.command {
            Commands::Serve(cmd) => {
                assert_eq!(cmd.host, "127.0.0.1");
                assert_eq!(cmd.port, 8080);
            }
            _ => panic!("expected Serve"),
        }
    }

    #[test]
    fn cli_parses_models() {
        let cli = Cli::parse_from(["sdx", "models"]);
        assert!(matches!(cli.command, Commands::Models));
    }

    #[test]
    fn cli_parses_global_config() {
        let cli = Cli::parse_from(["sdx", "--config", "/custom/config.toml", "models"]);
        assert_eq!(cli.config, Some(PathBuf::from("/custom/config.toml")));
    }

    #[test]
    fn cli_has_version_flag() {
        let cmd = Cli::command();
        // version is set via #[command(version)]
        assert!(cmd.get_version().is_some());
    }
}
