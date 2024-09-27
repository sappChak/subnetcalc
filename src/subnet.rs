use log::{debug, info};
use std::{net::Ipv4Addr, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Subnet {
    pub ip: Ipv4Addr,
    pub mask: u32,
}

impl Subnet {
    pub fn new(ip: Ipv4Addr, mask: u32) -> Self {
        Self { ip, mask }
    }

    /// Parse a subnet from a string in the format "IP/MASK"
    pub fn from_str(subnet: &str) -> Result<Self, &'static str> {
        let (ip_str, mask_str) = subnet.split_once('/').ok_or("Invalid subnet format")?;
        let ip = Ipv4Addr::from_str(ip_str).map_err(|_| "Invalid IP format")?;
        let mask = mask_str.parse::<u32>().map_err(|_| "Invalid mask format")?;
        info!("Parsed subnet: IP = {}, Mask = {}", ip, mask);
        Ok(Subnet::new(ip, mask))
    }

    /// Aggregate multiple subnets into the smallest common network
    pub fn aggregate(subnets: &[Subnet]) -> Result<Subnet, &'static str> {
        if subnets.is_empty() {
            return Err("Subnet list is empty");
        }
        if subnets.len() == 1 {
            info!("Single subnet provided: {:?}", subnets[0]);
            return Ok(subnets[0]);
        }

        let first_subnet = u32::from(subnets[0].ip) & Self::mask_to_u32(subnets[0].mask);
        debug!("First subnet (masked): {:032b}", first_subnet);

        let common_prefix = Self::calculate_common_prefix(subnets, first_subnet);
        debug!("Common prefix: {:032b}", common_prefix);

        let common_bits = Self::find_common_prefix_length(subnets);
        info!("Common prefix length: {}", common_bits);

        let aggregated_network = Ipv4Addr::from(common_prefix & (!0 << (32 - common_bits)));
        info!(
            "Aggregated network: IP = {}, Mask = {}",
            aggregated_network, common_bits
        );
        Ok(Subnet::new(aggregated_network, common_bits))
    }

    /// Convert the subnet mask length into a 32-bit mask
    pub fn mask_to_u32(mask: u32) -> u32 {
        let mask = !0 << (32 - mask);
        debug!("Mask to u32: {:032b}", mask);
        mask
    }

    /// Calculate the common prefix for a list of subnets
    fn calculate_common_prefix(subnets: &[Subnet], first_subnet: u32) -> u32 {
        subnets.iter().skip(1).fold(first_subnet, |acc, subnet| {
            let masked_ip = u32::from(subnet.ip) & Self::mask_to_u32(subnet.mask);
            debug!(
                "Current IP: {:032b}, Masked IP: {:032b}, Acc: {:032b}",
                u32::from(subnet.ip),
                masked_ip,
                acc
            );
            acc & masked_ip
        })
    }

    /// Find the length of the common prefix for a list of subnets
    pub fn find_common_prefix_length(subnets: &[Subnet]) -> u32 {
        let first_ip = u32::from(subnets[0].ip);
        (0..32)
            .rev()
            .take_while(|&i| {
                let mask = 1 << i;
                subnets
                    .iter()
                    .all(|subnet| (first_ip & mask) == (u32::from(subnet.ip) & mask))
            })
            .count() as u32
    }
}

impl std::fmt::Display for Subnet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.ip, self.mask)
    }
}

#[cfg(test)]
mod tests {
    use crate::subnet::Subnet;
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_subnet_valid() {
        let result = Subnet::from_str("192.168.100.0/27").unwrap();
        assert_eq!(result, Subnet::new(Ipv4Addr::new(192, 168, 100, 0), 27));

        let result = Subnet::from_str("10.0.0.0/8").unwrap();
        assert_eq!(result, Subnet::new(Ipv4Addr::new(10, 0, 0, 0), 8));
    }

    #[test]
    fn test_parse_subnet_invalid_format() {
        let result = Subnet::from_str("192.168.100.0-27");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid subnet format");

        let result = Subnet::from_str("invalid/27");
        assert!(result.is_err());
    }

    #[test]
    fn test_aggregate_subnets() {
        let subnets = vec![
            Subnet::new(Ipv4Addr::new(192, 168, 100, 0), 27),
            Subnet::new(Ipv4Addr::new(192, 168, 100, 32), 27),
            Subnet::new(Ipv4Addr::new(192, 168, 100, 64), 26),
        ];

        let result = Subnet::aggregate(&subnets).unwrap();
        assert_eq!(result, Subnet::new(Ipv4Addr::new(192, 168, 100, 0), 25));
    }

    #[test]
    fn test_aggregate_single_subnet() {
        let subnets = vec![Subnet::new(Ipv4Addr::new(192, 168, 100, 0), 27)];

        let result = Subnet::aggregate(&subnets).unwrap();
        assert_eq!(result, Subnet::new(Ipv4Addr::new(192, 168, 100, 0), 27)); // Single subnet stays the same
    }

    #[test]
    fn test_aggregate_subnets_empty() {
        let subnets: Vec<Subnet> = vec![];

        let result = Subnet::aggregate(&subnets);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Subnet list is empty");
    }

    #[test]
    fn test_find_common_prefix_length() {
        let subnets = vec![
            Subnet::new(Ipv4Addr::new(192, 168, 100, 0), 27),
            Subnet::new(Ipv4Addr::new(192, 168, 100, 32), 27),
            Subnet::new(Ipv4Addr::new(192, 168, 100, 64), 26),
        ];

        let result = Subnet::find_common_prefix_length(&subnets);
        assert_eq!(result, 25);
    }

    #[test]
    fn test_mask_to_u32() {
        assert_eq!(Subnet::mask_to_u32(24), 0xFFFFFF00); // /24 should give a mask of 255.255.255.0
        assert_eq!(Subnet::mask_to_u32(27), 0xFFFFFFE0); // /27 should give a mask of 255.255.255.224
    }
}
