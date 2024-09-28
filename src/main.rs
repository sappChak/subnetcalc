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
        Commands::Aggregate { subnets } => {
            let parsed_subnets: Vec<Subnet> = subnets
                .iter()
                .map(|s| Subnet::from_str(s))
                .collect::<Result<_, _>>()?;
            match Subnet::aggregate(&parsed_subnets) {
                Ok(aggregated_subnet) => println!(
                    "Aggregated subnet: {}",
                    aggregated_subnet.to_string().green()
                ),
                Err(e) => println!("Error: {}", e.to_string().red()),
            }
        }
        Commands::Info { subnet } => {
            let subnet = Subnet::from_str(subnet)?;
            println!("Subnet: {}", subnet.to_string().purple());
            println!("Netmask: {}", subnet.netmask().to_string().purple());
            println!("Wildcard: {}", subnet.wildcard().to_string().purple());
            println!("Broadcast: {}", subnet.broadcast().to_string().purple());
            println!("Hosts: {}", subnet.hosts().to_string().purple());
            println!("Class type: {}", subnet.class().to_string());
        }
    }

    Ok(())
}
