mod engine;

use clap::{Parser, Subcommand};
use engine::Engine;
use epsonia_checks::checks::check::Checks;

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

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.subcommand.unwrap() {
        SubCommands::Run { export, config } => run(export, config),
        _ => panic!("Subcommand not found"),
    }
}

fn run(export: &Option<String>, config: &Option<String>) {
    // let export = export.clone().unwrap_or(String::from("./export"));
    // let config = config.clone().unwrap_or(String::from("./config"));

    let checks: Vec<Checks> = Vec::new();
    Engine::new(checks, 100).run_engine();
}
