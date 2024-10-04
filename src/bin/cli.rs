use clap::{Parser, Subcommand};
use colored::Colorize;
use log::info;
use subnetcalc::subnet::Subnet;

#[derive(Parser)]
#[command(name = "subnetcalc")]
#[command(about = "A tool for subnet calculations", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Aggregate {
        #[arg(required = true)]
        subnets: Vec<String>,
    },
    Info {
        #[arg(required = true)]
        subnet: String,
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
        Command::Aggregate { subnets } => handle_aggregate(subnets),
        Command::Info { subnet } => handle_info(subnet),
    }
}

fn handle_aggregate(subnets: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_subnets = parse_subnets(subnets)?;
    match Subnet::aggregate(&parsed_subnets) {
        Ok(aggregated_subnet) => {
            info!(
                "Aggregated subnet: {}",
                aggregated_subnet.to_string().purple()
            );
        }
        Err(e) => {
            println!("Error: {}", e.to_string().red());
        }
    }
    Ok(())
}

fn handle_info(subnet: &str) -> Result<(), Box<dyn std::error::Error>> {
    let subnet = Subnet::from_str(subnet)?;
    display_subnet_info(&subnet);
    Ok(())
}

fn parse_subnets(subnets: &[String]) -> Result<Vec<Subnet>, Box<dyn std::error::Error>> {
    subnets
        .iter()
        .map(|s| Subnet::from_str(s))
        .collect::<Result<_, _>>()
        .map_err(|e| e.into())
}

fn display_subnet_info(subnet: &Subnet) {
    info!("Subnet: {}", subnet.to_string().purple());
    info!("Netmask: {}", subnet.netmask().to_string().purple());
    info!("Wildcard: {}", subnet.wildcard().to_string().purple());
    info!("Broadcast: {}", subnet.broadcast().to_string().purple());
    info!("Hosts: {}", subnet.hosts().to_string().purple());
    info!("Class type: {}", subnet.class().to_string());
}
