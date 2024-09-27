use clap::Parser;
use log::{error, info};
use std::env;
use subnetcalc::{
    cli::{Cli, Commands},
    subnet::Subnet,
};

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Aggregate { subnets } => {
            let parsed_subnets: Result<Vec<Subnet>, _> =
                subnets.iter().map(|s| Subnet::from_str(s)).collect();

            match parsed_subnets {
                Ok(subnets) => match Subnet::aggregate(&subnets) {
                    Ok(aggregated_subnet) => info!("Aggregated subnet: {}", aggregated_subnet),
                    Err(e) => error!("Error: {}", e),
                },
                Err(e) => error!("Invalid subnet format: {}", e),
            }
        }
        Commands::Info { subnet } => {
            todo!();
        }
    }
}
