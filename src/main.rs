mod cli;
mod data;
mod model;
mod report;
mod scheduler;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Schedule {
            jobs,
            signals,
            output,
            carbon_weight,
        } => {
            let jobs = data::load_jobs(&jobs)?;
            let signals = data::load_signals(&signals)?;

            println!("Loaded {} jobs.", jobs.len());
            println!("Loaded {} signal rows.", signals.len());

            let schedule = scheduler::schedule_jobs(&jobs, &signals, carbon_weight)?;

            report::write_schedule(&output, &schedule)?;

            println!("Wrote schedule to {}", output);

            report::print_summary(&schedule);
        }
    }

    Ok(())
}