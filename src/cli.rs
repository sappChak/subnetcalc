use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "subnetcalc")]
#[command(about = "A tool for subnet calculations", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Aggregate {
        #[arg(required = true)]
        subnets: Vec<String>,
    },
    Info {
        #[arg(required = true)]
        subnet: String,
    },
}
