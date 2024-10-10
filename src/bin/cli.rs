use clap::{Parser, Subcommand};
use colored::*;
use std::str::FromStr;
use subnetcalc::subnet::Network;

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
        /// Number of required hosts
        #[arg(required = true)]
        hosts: u32,
        /// Number of required networks
        #[arg(required = true)]
        networks_number: u32,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    #[cfg(debug_assertions)]
    let args = vec![
        "subnetcalc".to_string(),
        "aggregate".to_string(),
        "192.168.1.0/27".to_string(),
        "192.168.1.32/27".to_string(),
        "192.168.1.64/26".to_string(),
    ];

    #[cfg(not(debug_assertions))]
    let args: Vec<String> = std::env::args().collect();

    let cli = Cli::parse_from(args);

    match &cli.command {
        Commands::Aggregate { networks } => handle_aggregate(networks),
        Commands::Info { network } => handle_info(network),
        Commands::Mask {
            network,
            hosts,
            networks_number: networks,
        } => handle_mask(network, *hosts, *networks),
    }
}

fn handle_aggregate(networks: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_networks: Vec<Network> = parse_networks(networks)?;
    match Network::aggregate_networks(&parsed_networks) {
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
    let network = Network::from_str(network_str)?;
    display_network_info(&network);
    Ok(())
}

fn handle_mask(
    network: &str,
    required_hosts: u32,
    required_networks: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_network = Network::from_str(network)?;
    match Network::determine_subnet_mask(parsed_network.mask, required_hosts, required_networks) {
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

fn parse_networks(networks: &[String]) -> Result<Vec<Network>, Box<dyn std::error::Error>> {
    networks
        .iter()
        .map(|s| Network::from_str(s).map_err(|e| e.into()))
        .collect::<Result<Vec<_>, _>>()
}

fn display_network_info(network: &Network) {
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
