// crates/allure-cli/src/main.rs
use allure_generator::generate_report;
use allure_server::serve_report;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "allure-rs", version = "0.1.0", about = "Allure Report in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate the report
    Generate {
        /// Directory containing test result files
        #[arg(short, long, default_value = "allure-results")]
        results: PathBuf,

        /// Output directory for the report
        #[arg(short, long, default_value = "allure-report")]
        output: PathBuf,
    },
    /// Serve the report
    Serve {
        /// Directory containing the generated report
        #[arg(short, long, default_value = "allure-report")]
        report: PathBuf,

        /// Port to serve the report on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { results, output } => {
            // Load plugins (placeholder)
            let plugins: Vec<Box<dyn allure_core::Plugin>> = Vec::new();
            generate_report(results, output, &plugins)?;
        }
        Commands::Serve { report, port } => {
            serve_report(report, port).await?;
        }
    }

    Ok(())
}
