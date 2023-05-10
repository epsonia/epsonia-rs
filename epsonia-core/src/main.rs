mod checks_config;
mod config;
mod engine;
mod models;

use clap::{Parser, Subcommand};
use engine::Engine;

use epsonia_checks::check::Check;

#[derive(Parser)]
#[command(
    author = "matees",
    version = "0.1.0",
    about = "Epsonia CLI",
    long_about = "Epsonia CLI (long)"
)]
struct Cli {
    // Subcommands
    #[command(subcommand)]
    subcommand: Option<SubCommands>,
}

#[derive(Subcommand)]
enum SubCommands {
    Run {
        // Export folder - Optional
        #[arg(
            short,
            long = "Manually set the export folder, default is ./export",
            value_name = "export"
        )]
        export: Option<String>,

        // Config folder - Optional
        #[arg(
            short,
            long = "Manually set the config folder, default is ./config",
            value_name = "config"
        )]
        config: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli: Cli = Cli::parse();

    match &cli.subcommand.unwrap() {
        SubCommands::Run { export, config } => run(export, config),
    }
    .await
}

async fn run(export: &Option<String>, config: &Option<String>) {
    let config_path: String = config.clone().unwrap_or(String::from("./config"));

    let checks: &Vec<Check> = &checks_config::get_checks();
    let config: config::Config = config::Config::get(config_path);

    let mut engine: Engine = Engine::new(
        checks.to_vec(),
        checks_config::get_max_points(&checks),
        if let Some(export) = export {
            config::Config {
                auto_export_path: export.clone(),
                ..config
            }
        } else {
            config.clone()
        },
    );
    let mut interval: tokio::time::Interval = tokio::time::interval(
        tokio::time::Duration::from_secs(config.engine_interval as u64),
    );

    loop {
        interval.tick().await;
        looop(&mut engine);
    }
}

fn looop(engine: &mut Engine) {
    engine.run_engine();
}
