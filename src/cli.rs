use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "carbon-aware-scheduler")]
#[command(about = "Schedule flexible data-center workloads for lower cost and carbon emissions")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Schedule {
        #[arg(long)]
        jobs: String,

        #[arg(long)]
        signals: String,

        #[arg(long)]
        output: String,

        #[arg(long, default_value_t = 0.05)]
        carbon_weight: f64,
    },
}