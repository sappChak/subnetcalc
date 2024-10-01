use clap::{Parser, Subcommand};
use colored::Colorize;
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

    let cli = Cli::parse();

    match &cli.command {
        Command::Aggregate { subnets } => handle_aggregate(subnets),
        Command::Info { subnet } => handle_info(subnet),
    }
}

fn handle_aggregate(subnets: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_subnets = parse_subnets(subnets)?;
    match Subnet::aggregate(&parsed_subnets) {
        Ok(aggregated_subnet) => {
            println!(
                "Aggregated subnet: {}",
                aggregated_subnet.to_string().green()
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
    println!("Subnet: {}", subnet.to_string().purple());
    println!("Netmask: {}", subnet.netmask().to_string().purple());
    println!("Wildcard: {}", subnet.wildcard().to_string().purple());
    println!("Broadcast: {}", subnet.broadcast().to_string().purple());
    println!("Hosts: {}", subnet.hosts().to_string().purple());
    println!("Class type: {}", subnet.class().to_string());
}
