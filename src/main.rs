use clap::Parser;
use log::{error, info};
use std::{env, net::Ipv4Addr};
use subnetcalc::{
    aggregate_subnets,
    cli::{Cli, Commands},
    parse_subnet,
};

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Aggregate { subnets } => {
            let parsed_subnets: Result<Vec<(Ipv4Addr, u32)>, _> =
                subnets.iter().map(|s| parse_subnet(s)).collect();

            match parsed_subnets {
                Ok(subnets) => match aggregate_subnets(&subnets) {
                    Ok((ip, mask)) => info!("Aggregated subnet: {}/{}", ip, mask),
                    Err(e) => error!("Error: {}", e),
                },
                Err(e) => error!("Invalid subnet format: {}", e),
            }
        }
    }
}
