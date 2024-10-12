mod checks_config;
mod config;
mod engine;
mod models;

use clap::{Parser, Subcommand};
use engine::Engine;
use std::error::Error;

#[derive(Parser)]
#[command(
    author = "matees",
    version = "0.1.0",
    about = "Epsonia: A cybersecurity scoring engine for practice and competition",
    long_about = "Epsonia is a scoring engine based on the CyberPatriot competition. It searches an environment for vulnerabilities and scores based on their status (fixed, not fixed, or penalized)."
)]
struct Cli {
    #[command(subcommand)]
    subcommand: Option<SubCommands>,
}

#[derive(Subcommand)]
enum SubCommands {
    Run {
        #[arg(
            short,
            long,
            value_name = "EXPORT_PATH",
            help = "Set the export folder for scoring reports (default: ./export)",
            long_help = "Specify a custom path to export the scoring reports. This overrides the auto_export_path in the config file."
        )]
        export: Option<String>,

        #[arg(
            short,
            long,
            value_name = "CONFIG_PATH",
            help = "Set the configuration folder (default: ./config)",
            long_help = "Specify the path to the folder containing config.json and checks.json files. This allows for multiple configuration setups."
        )]
        config: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::parse();

    match cli.subcommand {
        Some(SubCommands::Run { export, config }) => run(&export, &config).await?,
        None => {
            eprintln!("Invalid CLI usage. Please use help command to view a list of commands!");
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn run(export: &Option<String>, config: &Option<String>) -> Result<(), Box<dyn Error>> {
    let config_path: String = config.clone().unwrap_or_else(|| String::from("./config"));

    let (checks, hidden_penalties) = checks_config::get_checks()?;
    let config: config::Config = config::Config::get(&config_path)?;

    let mut engine: Engine = Engine::new(
        checks.clone(),
        hidden_penalties,
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
        if let Err(e) = engine.run_engine() {
            eprintln!("Error running engine: {}", e);
        }
    }
}
