pub mod cli;
use log::{debug, info};
use std::{net::Ipv4Addr, str::FromStr};

pub fn parse_subnet(subnet: &str) -> Result<(Ipv4Addr, u32), &'static str> {
    let (ip_str, mask_str) = subnet.split_once('/').ok_or("Invalid subnet format")?;
    let ip = Ipv4Addr::from_str(ip_str).map_err(|_| "Invalid IP format")?;
    let mask = mask_str.parse::<u32>().map_err(|_| "Invalid mask format")?;
    info!("Parsed subnet: IP = {}, Mask = {}", ip, mask);
    Ok((ip, mask))
}

pub fn aggregate_subnets(subnets: &[(Ipv4Addr, u32)]) -> Result<(Ipv4Addr, u32), &'static str> {
    if subnets.is_empty() {
        return Err("Subnet list is empty");
    }
    if subnets.len() == 1 {
        info!("Single subnet provided: {:?}", subnets[0]);
        return Ok(subnets[0]);
    }

    let first_subnet = u32::from(subnets[0].0) & mask_to_u32(subnets[0].1);
    debug!("First subnet (masked): {:032b}", first_subnet);
    let common_prefix = calculate_common_prefix(subnets, first_subnet);
    debug!("Common prefix: {:032b}", common_prefix);
    let common_bits = find_common_prefix_length(subnets);
    info!("Common prefix length: {}", common_bits);
    let aggregated_network = Ipv4Addr::from(common_prefix & (!0 << (32 - common_bits)));
    info!(
        "Aggregated network: IP = {}, Mask = {}",
        aggregated_network, common_bits
    );
    Ok((aggregated_network, common_bits))
}

pub fn calculate_common_prefix(subnets: &[(Ipv4Addr, u32)], first_subnet: u32) -> u32 {
    subnets
        .iter()
        .skip(1)
        .fold(first_subnet, |acc, &(ip, mask)| {
            let masked_ip = u32::from(ip) & mask_to_u32(mask);
            debug!(
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
            subnets
                .iter()
                .all(|&(ip, _)| (first_ip & mask) == (u32::from(ip) & mask))
        })
        .count() as u32
}

pub fn mask_to_u32(mask: u32) -> u32 {
    let mask = !0 << (32 - mask);
    debug!("Mask to u32: {:032b}", mask);
    mask
}
