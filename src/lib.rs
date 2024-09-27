use clap::Parser;
use std::{net::Ipv4Addr, str::FromStr};

#[derive(Parser)]
pub struct Cli {
    pub subnets: Vec<String>, // A list of subnets
}

// Parses a subnet string into an (Ipv4Addr, u32) tuple
pub fn parse_subnet(subnet: &str) -> Result<(Ipv4Addr, u32), &'static str> {
    let parts: Vec<&str> = subnet.split('/').collect();
    if parts.len() != 2 {
        return Err("Invalid subnet format");
    }
    let ip = Ipv4Addr::from_str(parts[0]).expect("Invalid IP format");
    let mask = parts[1].parse::<u32>().expect("Invalid mask format");
    Ok((ip, mask))
}

// Aggregates a list of subnets into a common network address and prefix length
pub fn aggregate_subnets(subnets: &[(Ipv4Addr, u32)]) -> Result<(Ipv4Addr, u32), &'static str> {
    if subnets.is_empty() {
        return Err("Subnet list is empty");
    }

    // Convert the first subnet IP to u32
    let first_subnet = ip_to_u32(subnets[0].0) & mask_to_u32(subnets[0].1);
    println!("First subnet: {:032b}", first_subnet);

    // Calculate the common prefix across all subnets
    let common_prefix = subnets
        .iter()
        .skip(1)
        .fold(first_subnet, |acc, &(ip, mask)| {
            let ip_u32 = ip_to_u32(ip) & mask_to_u32(mask);
            println!("Current IP: {:032b}", ip_u32);
            acc & ip_u32
        });
    println!("Common prefix: {:032b}", common_prefix);

    // Calculate the number of common prefix bits
    let common_bits = find_common_prefix_length(subnets);
    println!("Common bits: {}", common_bits);

    let aggregated_network = Ipv4Addr::from(common_prefix & (!0 << (32 - common_bits)));
    println!("Aggregated network: {}", aggregated_network);

    Ok((aggregated_network, common_bits))
}

// Converts an IPv4 address to a u32
fn ip_to_u32(ip: Ipv4Addr) -> u32 {
    u32::from(ip)
}

// Converts a mask length to a u32
fn mask_to_u32(mask: u32) -> u32 {
    (!0 << (32 - mask)) & 0xFFFFFFFF
}

// Finds the number of common prefix bits across all subnets
fn find_common_prefix_length(subnets: &[(Ipv4Addr, u32)]) -> u32 {
    let mut prefix_len = 0;
    let first_ip = ip_to_u32(subnets[0].0);

    for i in (0..32).rev() {
        let mask = 1 << i;
        if subnets
            .iter()
            .all(|&(ip, _)| (first_ip & mask) == (ip_to_u32(ip) & mask))
        {
            prefix_len += 1;
        } else {
            break;
        }
    }

    prefix_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_subnet_valid() {
        let result = parse_subnet("192.168.100.0/27").unwrap();
        assert_eq!(result, (Ipv4Addr::new(192, 168, 100, 0), 27));

        let result = parse_subnet("10.0.0.0/8").unwrap();
        assert_eq!(result, (Ipv4Addr::new(10, 0, 0, 0), 8));
    }

    #[test]
    fn test_parse_subnet_invalid_format() {
        let result = parse_subnet("192.168.100.0-27");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid subnet format");

        let result = parse_subnet("invalid/27");
        assert!(result.is_err());
    }

    #[test]
    fn test_aggregate_subnets() {
        let subnets = vec![
            (Ipv4Addr::new(192, 168, 100, 0), 27),
            (Ipv4Addr::new(192, 168, 100, 32), 27),
            (Ipv4Addr::new(192, 168, 100, 64), 26),
        ];

        let result = aggregate_subnets(&subnets).unwrap();
        assert_eq!(result, (Ipv4Addr::new(192, 168, 100, 0), 25)); // Expect aggregation to /25
    }

    #[test]
    fn test_aggregate_single_subnet() {
        let subnets = vec![(Ipv4Addr::new(192, 168, 100, 0), 27)];

        let result = aggregate_subnets(&subnets).unwrap();
        assert_eq!(result, (Ipv4Addr::new(192, 168, 100, 0), 27)); // Single subnet stays the same
    }

    #[test]
    fn test_aggregate_subnets_empty() {
        let subnets = vec![];

        let result = aggregate_subnets(&subnets);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Subnet list is empty");
    }

    #[test]
    fn test_find_common_prefix_length() {
        let subnets = vec![
            (Ipv4Addr::new(192, 168, 100, 0), 27),
            (Ipv4Addr::new(192, 168, 100, 32), 27),
            (Ipv4Addr::new(192, 168, 100, 64), 26),
        ];

        let result = find_common_prefix_length(&subnets);
        assert_eq!(result, 25); // Common prefix length is 25
    }

    #[test]
    fn test_ip_to_u32() {
        let ip = Ipv4Addr::new(192, 168, 100, 0);
        assert_eq!(ip_to_u32(ip), 0xC0A86400); // 192.168.100.0 in u32
    }

    #[test]
    fn test_mask_to_u32() {
        assert_eq!(mask_to_u32(24), 0xFFFFFF00); // /24 should give a mask of 255.255.255.0
        assert_eq!(mask_to_u32(27), 0xFFFFFFE0); // /27 should give a mask of 255.255.255.224
    }
}
