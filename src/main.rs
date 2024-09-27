use clap::Parser;
use log::error;
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
                Ok(aggregated_subnet) => println!("Aggregated subnet: {}", aggregated_subnet),
                Err(e) => error!("Error: {}", e),
            }
        }
        Commands::Info { subnet } => {
            let subnet = Subnet::from_str(subnet)?;
            println!("Subnet: {}", subnet);
            println!("Netmask: {}", subnet.netmask());
            println!("Wildcard: {}", subnet.wildcard());
            println!("Broadcast: {}", subnet.broadcast());
        }
    }

    Ok(())
}
