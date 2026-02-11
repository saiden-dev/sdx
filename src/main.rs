use std::path::Path;
use std::process;

use clap::Parser;

use sdx::{AppConfig, Cli, Commands, GenerateArgs, build_router};

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn run(cli: Cli) -> sdx::Result<()> {
    let config_path = cli.config.unwrap_or_else(AppConfig::default_config_path);
    let config = AppConfig::load(&config_path)?;
    let sd_cli_path = config.resolve_sd_cli_path()?;

    match cli.command {
        Commands::Generate(cmd) => cmd_generate(&sd_cli_path, &config, cmd),
        Commands::Serve(cmd) => cmd_serve(config, sd_cli_path, cmd),
        Commands::Models => cmd_models(&config),
    }
}

fn cmd_generate(sd_cli_path: &Path, config: &AppConfig, cmd: sdx::GenerateCmd) -> sdx::Result<()> {
    let model_name = config.resolve_model_name(cmd.model.as_deref())?;
    let model_config = config.resolve_model(model_name)?;
    let mut args = GenerateArgs::from_model_config(sd_cli_path, model_config);

    args.prompt = cmd.prompt;
    args.output = cmd.output;

    if let Some(neg) = cmd.negative_prompt {
        args.negative_prompt = Some(neg);
    }
    if let Some(w) = cmd.width {
        args.width = w;
    }
    if let Some(h) = cmd.height {
        args.height = h;
    }
    if let Some(s) = cmd.steps {
        args.steps = s;
    }
    if let Some(c) = cmd.cfg_scale {
        args.cfg_scale = c;
    }
    if let Some(g) = cmd.guidance {
        args.guidance = Some(g);
    }
    if let Some(s) = cmd.seed {
        args.seed = s;
    }
    if let Some(m) = cmd.sampler {
        args.sampling_method = m;
    }
    if let Some(s) = cmd.scheduler {
        args.scheduler = s;
    }
    if let Some(b) = cmd.batch_count {
        args.batch_count = b;
    }

    let output_path = args.run()?;
    println!("{}", output_path.display());
    Ok(())
}

fn cmd_serve(
    config: AppConfig,
    sd_cli_path: std::path::PathBuf,
    cmd: sdx::ServeCmd,
) -> sdx::Result<()> {
    let addr = format!("{}:{}", cmd.host, cmd.port);
    eprintln!("listening on http://{addr}");

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let app = build_router(config, sd_cli_path);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    })
}

fn cmd_models(config: &AppConfig) -> sdx::Result<()> {
    if config.models.is_empty() {
        println!("no models configured");
        return Ok(());
    }

    for (name, model) in &config.models {
        println!("{name}:");
        for (label, path) in model.model_paths() {
            let exists = if path.exists() { "ok" } else { "MISSING" };
            println!("  {label}: {} [{exists}]", path.display());
        }

        let mut defaults = Vec::new();
        if let Some(w) = model.width {
            defaults.push(format!("{w}x{}", model.height.unwrap_or(w)));
        }
        if let Some(s) = model.steps {
            defaults.push(format!("{s} steps"));
        }
        if let Some(m) = &model.sampling_method {
            defaults.push(m.clone());
        }
        if !defaults.is_empty() {
            println!("  defaults: {}", defaults.join(", "));
        }
        println!();
    }
    Ok(())
}
