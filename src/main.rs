use clap::Parser;
use std::net::Ipv4Addr;
use subnetcalc::{
    aggregate_subnets,
    cli::{Cli, Commands},
    parse_subnet,
};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Aggregate { subnets } => {
            let parsed_subnets: Vec<(Ipv4Addr, u32)> = subnets
                .iter()
                .map(|s| parse_subnet(s).expect("Invalid subnet format"))
                .collect();
            match aggregate_subnets(&parsed_subnets) {
                Ok((ip, mask)) => println!("Aggregated subnet: {}/{}", ip, mask),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
