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
    /// Aggregate multiple routes into one larger route
    Aggregate {
        /// List of routes to aggregate (in CIDR notation)
        #[arg(required = true)]
        routes: Vec<String>,
    },
    /// Display information about a specific route
    Info {
        /// Network to display information for (in CIDR notation)
        #[arg(required = true)]
        route: String,
    },
    /// Calculate the mask for a given number of hosts and networks
    Mask {
        // Route to calculate the mask for
        #[arg(required = true)]
        route: String,
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
        Commands::Aggregate { routes: networks } => handle_aggregate(networks),
        Commands::Info { route: network } => handle_info(network),
        Commands::Mask {
            route: network,
            hosts,
            subnets_number: networks,
        } => handle_mask(network, *hosts, *networks),
    }
}

fn handle_aggregate(routes: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_routes: Vec<Route> = parse_routes(routes)?;
    match aggregate_routes(&parsed_routes) {
        Ok(aggregated) => {
            println!(
                "{}: {}",
                "Aggregated Route".bold().green(),
                aggregated.to_string().purple()
            );
        }
        Err(e) => {
            println!("{}: {}", "Error".bold().red(), e.to_string().red());
        }
    }
    Ok(())
}

fn handle_info(route: &str) -> Result<(), Box<dyn std::error::Error>> {
    let route = Route::from_str(route)?;
    display_network_info(&route);
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

fn parse_routes(routes: &[String]) -> Result<Vec<Route>, Box<dyn std::error::Error>> {
    routes
        .iter()
        .map(|s| Route::from_str(s).map_err(|e| e.into()))
        .collect::<Result<Vec<_>, _>>()
}

fn display_network_info(route: &Route) {
    println!("{}: {}", "Route".bold().green(), route.to_string().purple());
    println!(
        "{}: {}",
        "Netmask".bold().green(),
        route.netmask_address().to_string().yellow()
    );
    println!(
        "{}: {}",
        "Wildcard".bold().green(),
        route.wildcard_address().to_string().yellow()
    );
    println!(
        "{}: {}",
        "Broadcast".bold().green(),
        route.broadcast_address().to_string().yellow()
    );
    println!(
        "{}: {}",
        "Available Hosts".bold().green(),
        route.available_hosts().to_string().yellow()
    );
    println!(
        "{}: {}",
        "Class".bold().green(),
        route.ip_class().to_string().cyan()
    );
}
