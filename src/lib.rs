mod api;
mod cli;
mod config;
mod embedded;
mod error;
mod sd_cli;
mod server;

pub use cli::{Cli, Commands, GenerateCmd, ServeCmd};
pub use config::{AppConfig, ModelConfig};
pub use error::{Error, Result};
pub use sd_cli::GenerateArgs;
pub use server::build_router;
