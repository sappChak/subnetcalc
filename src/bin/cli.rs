use clap::{Parser, Subcommand};
use colored::*;
use std::str::FromStr;
use subnetcalc::routes::{aggregate_routes, determine_subnet_mask, Route};

#[derive(Parser)]
#[command(name = "subnetcalc", about = "A tool for subnet calculations")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Aggregate multiple networks into one larger network
    Aggregate {
        /// List of networks to aggregate (in CIDR notation)
        #[arg(required = true)]
        networks: Vec<String>,
    },
    /// Display information about a specific network
    Info {
        /// Network to display information for (in CIDR notation)
        #[arg(required = true)]
        network: String,
    },
    /// Calculate the mask for a given number of hosts and networks
    Mask {
        // Network to calculate the mask for
        #[arg(required = true)]
        network: String,
        /// Number of required networks
        #[arg(required = true)]
        subnets_number: u32,
        /// Number of required hosts
        #[arg(required = true)]
        hosts: u32,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let cli = Cli::parse_from(args);

    match &cli.command {
        Commands::Aggregate { networks } => handle_aggregate(networks),
        Commands::Info { network } => handle_info(network),
        Commands::Mask {
            network,
            hosts,
            subnets_number: networks,
        } => handle_mask(network, *hosts, *networks),
    }
}

fn handle_aggregate(networks: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_networks: Vec<Route> = parse_networks(networks)?;
    match aggregate_routes(&parsed_networks) {
        Ok(aggregated_network) => {
            println!(
                "{}: {}",
                "Aggregated Network".bold().green(),
                aggregated_network.to_string().purple()
            );
        }
        Err(e) => {
            println!("{}: {}", "Error".bold().red(), e.to_string().red());
        }
    }
    Ok(())
}

fn handle_info(network_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let network = Route::from_str(network_str)?;
    display_network_info(&network);
    Ok(())
}

fn handle_mask(
    network: &str,
    required_hosts: u32,
    required_networks: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_network = Route::from_str(network)?;
    match determine_subnet_mask(parsed_network.prefix, required_networks, required_hosts) {
        Ok(mask) => {
            println!(
                "{}: {}",
                "Subnet Mask".bold().green(),
                mask.to_string().yellow()
            );
        }
        Err(e) => {
            println!("{}: {}", "Error".bold().red(), e.to_string().red());
        }
    }
    Ok(())
}

fn parse_networks(networks: &[String]) -> Result<Vec<Route>, Box<dyn std::error::Error>> {
    networks
        .iter()
        .map(|s| Route::from_str(s).map_err(|e| e.into()))
        .collect::<Result<Vec<_>, _>>()
}

fn display_network_info(network: &Route) {
    println!(
        "{}: {}",
        "Network".bold().green(),
        network.to_string().purple()
    );
    println!(
        "{}: {}",
        "Netmask".bold().green(),
        network.netmask_address().to_string().yellow()
    );
    println!(
        "{}: {}",
        "Wildcard".bold().green(),
        network.wildcard_address().to_string().yellow()
    );
    println!(
        "{}: {}",
        "Broadcast".bold().green(),
        network.broadcast_address().to_string().yellow()
    );
    println!(
        "{}: {}",
        "Available Hosts".bold().green(),
        network.available_hosts().to_string().yellow()
    );
    println!(
        "{}: {}",
        "Class".bold().green(),
        network.ip_class().to_string().cyan()
    );
}
