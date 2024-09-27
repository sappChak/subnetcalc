use clap::Parser;
use std::{net::Ipv4Addr, str::FromStr};

#[derive(Parser)]
pub struct Cli {
    pub subnets: Vec<String>,
}

pub fn parse_subnet(subnet: &str) -> Result<(Ipv4Addr, u32), &'static str> {
    let (ip_str, mask_str) = match subnet.split_once('/') {
        Some(parts) => parts,
        None => return Err("Invalid subnet format"),
    };

    let ip = Ipv4Addr::from_str(ip_str).map_err(|_| "Invalid IP format")?;
    let mask = mask_str.parse::<u32>().map_err(|_| "Invalid mask format")?;

    Ok((ip, mask))
}

pub fn aggregate_subnets(subnets: &[(Ipv4Addr, u32)]) -> Result<(Ipv4Addr, u32), &'static str> {
    match subnets {
        [] => Err("Subnet list is empty"),
        [single_subnet] => Ok(*single_subnet),
        _ => {
            let first_subnet = u32::from(subnets[0].0) & mask_to_u32(subnets[0].1);
            let common_prefix = calculate_common_prefix(subnets, first_subnet);
            let common_bits = find_common_prefix_length(subnets);
            let aggregated_network = Ipv4Addr::from(common_prefix & (!0 << (32 - common_bits)));
            Ok((aggregated_network, common_bits))
        }
    }
}

fn calculate_common_prefix(subnets: &[(Ipv4Addr, u32)], first_subnet: u32) -> u32 {
    subnets
        .iter()
        .skip(1)
        .fold(first_subnet, |acc, &(ip, mask)| {
            acc & (u32::from(ip) & mask_to_u32(mask))
        })
}

fn find_common_prefix_length(subnets: &[(Ipv4Addr, u32)]) -> u32 {
    let first_ip = u32::from(subnets[0].0);
    (0..32)
        .rev()
        .take_while(|&i| {
            let mask = 1 << i;
            subnets
                .iter()
                .all(|&(ip, _)| (first_ip & mask) == (u32::from(ip) & mask))
        })
        .count() as u32
}

fn mask_to_u32(mask: u32) -> u32 {
    (!0 << (32 - mask)) & 0xFFFFFFFF
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
        assert_eq!(result, (Ipv4Addr::new(192, 168, 100, 0), 25));
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
        assert_eq!(result, 25);
    }

    #[test]
    fn test_mask_to_u32() {
        assert_eq!(mask_to_u32(24), 0xFFFFFF00); // /24 should give a mask of 255.255.255.0
        assert_eq!(mask_to_u32(27), 0xFFFFFFE0); // /27 should give a mask of 255.255.255.224
    }
}
