pub mod cli;
use std::{net::Ipv4Addr, str::FromStr};

pub fn parse_subnet(subnet: &str) -> Result<(Ipv4Addr, u32), &'static str> {
    let (ip_str, mask_str) = match subnet.split_once('/') {
        Some(parts) => parts,
        None => return Err("Invalid subnet format"),
    };
    let ip = Ipv4Addr::from_str(ip_str).map_err(|_| "Invalid IP format")?;
    let mask = mask_str.parse::<u32>().map_err(|_| "Invalid mask format")?;
    println!("Parsed subnet: IP = {}, Mask = {}", ip, mask);
    Ok((ip, mask))
}

pub fn aggregate_subnets(subnets: &[(Ipv4Addr, u32)]) -> Result<(Ipv4Addr, u32), &'static str> {
    match subnets {
        [] => Err("Subnet list is empty"),
        [single_subnet] => {
            println!("Single subnet provided: {:?}", single_subnet);
            Ok(*single_subnet)
        }
        _ => {
            let first_subnet = u32::from(subnets[0].0) & mask_to_u32(subnets[0].1);
            println!("First subnet (masked): {:032b}", first_subnet);
            let common_prefix = calculate_common_prefix(subnets, first_subnet);
            println!("Common prefix: {:032b}", common_prefix);
            let common_bits = find_common_prefix_length(subnets);
            println!("Common prefix length: {}", common_bits);
            let aggregated_network = Ipv4Addr::from(common_prefix & (!0 << (32 - common_bits)));
            println!(
                "Aggregated network: IP = {}, Mask = {}",
                aggregated_network, common_bits
            );
            Ok((aggregated_network, common_bits))
        }
    }
}

pub fn calculate_common_prefix(subnets: &[(Ipv4Addr, u32)], first_subnet: u32) -> u32 {
    subnets
        .iter()
        .skip(1)
        .fold(first_subnet, |acc, &(ip, mask)| {
            let masked_ip = u32::from(ip) & mask_to_u32(mask);
            println!(
                "Current IP: {:032b}, Masked IP: {:032b}, Acc: {:032b}",
                u32::from(ip),
                masked_ip,
                acc
            );
            acc & masked_ip
        })
}

pub fn find_common_prefix_length(subnets: &[(Ipv4Addr, u32)]) -> u32 {
    let first_ip = u32::from(subnets[0].0);
    (0..32)
        .rev()
        .take_while(|&i| {
            let mask = 1 << i;
            let all_match = subnets
                .iter()
                .all(|&(ip, _)| (first_ip & mask) == (u32::from(ip) & mask));
            println!(
                "Bit position: {}, Mask: {:032b}, All match: {}",
                i, mask, all_match
            );
            all_match
        })
        .count() as u32
}

pub fn mask_to_u32(mask: u32) -> u32 {
    let mask = !0 << (32 - mask);
    println!("Mask to u32: {:032b}", mask);
    mask
}
