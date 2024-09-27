use clap::Parser;
use std::net::Ipv4Addr;
use subnetcalc::{aggregate_subnets, parse_subnet, Cli};

fn main() {
    let args = Cli::parse();

    // Parse subnets from input
    let subnets: Vec<(Ipv4Addr, u32)> = args
        .subnets
        .iter()
        .map(|s| parse_subnet(s).expect("Invalid subnet format"))
        .collect();

    // Calculate the aggregated subnet
    match aggregate_subnets(&subnets) {
        Ok((aggregated_network, aggregated_mask)) => {
            println!(
                "Aggregated subnet: {}/{}",
                aggregated_network, aggregated_mask
            );
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
