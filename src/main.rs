use std::path::Path;
use std::process;
use std::time::Duration;

use clap::{CommandFactory, Parser};
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, Color, Table};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use sdx::{AppConfig, Cli, Commands, GenerateArgs, build_router};

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("{} {e}", style("error:").red().bold());
        process::exit(1);
    }
}

fn run(cli: Cli) -> sdx::Result<()> {
    let Some(command) = cli.command else {
        Cli::command().print_help().expect("can write to stdout");
        println!();
        return Ok(());
    };

    let config_path = cli.config.unwrap_or_else(AppConfig::default_config_path);
    let config = AppConfig::load(&config_path)?;
    let sd_cli_path = config.resolve_sd_cli_path()?;

    match command {
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

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .expect("valid template"),
    );
    spinner.set_message(format!("Generating with {model_name}..."));
    spinner.enable_steady_tick(Duration::from_millis(80));

    let result = args.run();

    match &result {
        Ok(path) => {
            spinner.finish_and_clear();
            eprintln!(
                "{} {}",
                style("✓").green().bold(),
                style(path.display()).bold(),
            );
        }
        Err(_) => {
            spinner.finish_and_clear();
            eprintln!("{} generation failed", style("✗").red().bold());
        }
    }

    result.map(|_| ())
}

fn cmd_serve(
    config: AppConfig,
    sd_cli_path: std::path::PathBuf,
    cmd: sdx::ServeCmd,
) -> sdx::Result<()> {
    let addr = format!("{}:{}", cmd.host, cmd.port);
    eprintln!(
        "{} listening on {}",
        style("sdx").bold(),
        style(format!("http://{addr}")).cyan().underlined(),
    );

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
        eprintln!("{}", style("no models configured").dim());
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_header(["Name", "Type", "Files", "Status", "Defaults"]);

    let mut names: Vec<&String> = config.models.keys().collect();
    names.sort();

    for name in names {
        let model = &config.models[name.as_str()];
        let model_type = if model.has_component_paths() {
            "component"
        } else {
            "single"
        };

        let paths = model.model_paths();

        let files: String = paths
            .iter()
            .map(|(label, _)| *label)
            .collect::<Vec<_>>()
            .join(", ");

        let all_ok = paths.iter().all(|(_, p)| p.exists());
        let status_cell = if all_ok {
            Cell::new("ok").fg(Color::Green)
        } else {
            Cell::new("MISSING").fg(Color::Red)
        };

        let mut defaults = Vec::new();
        if let Some(w) = model.width {
            defaults.push(format!("{}x{}", w, model.height.unwrap_or(w)));
        }
        if let Some(s) = model.steps {
            defaults.push(format!("{s} steps"));
        }
        if let Some(m) = &model.sampling_method {
            defaults.push(m.clone());
        }

        table.add_row(vec![
            Cell::new(name),
            Cell::new(model_type),
            Cell::new(files),
            status_cell,
            Cell::new(defaults.join(", ")),
        ]);
    }

    println!("{table}");
    Ok(())
}
