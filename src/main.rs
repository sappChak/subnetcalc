use clap::Parser;
use colored::Colorize;
use subnetcalc::{
    cli::{Cli, Commands},
    subnet::Subnet,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Aggregate { subnets } => handle_aggregate(subnets),
        Commands::Info { subnet } => handle_info(subnet),
    }
}

fn handle_aggregate(subnets: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_subnets: Vec<Subnet> = parse_subnets(subnets)?;
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

fn handle_info(subnet: &String) -> Result<(), Box<dyn std::error::Error>> {
    let subnet = Subnet::from_str(subnet)?;
    display_subnet_info(&subnet);
    Ok(())
}

fn parse_subnets(subnets: &Vec<String>) -> Result<Vec<Subnet>, Box<dyn std::error::Error>> {
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
