use log::info;
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

    pub fn from_str(subnet: &str) -> Result<Self, &'static str> {
        if let Some((ip_str, mask_str)) = subnet.split_once('/') {
            let ip = Ipv4Addr::from_str(ip_str).map_err(|_| "Invalid IP format")?;
            let mask = mask_str.parse::<u32>().map_err(|_| "Invalid mask format")?;
            info!("Parsed subnet: IP = {}, Mask = {}", ip, mask);
            Ok(Subnet::new(ip, mask))
        } else {
            let ip = Ipv4Addr::from_str(subnet).map_err(|_| "Invalid IP format")?;
            let mask = Self::default_mask(ip);
            info!("No prefix provided, assuming default mask based on class. Parsed IP = {}, Mask = {}", ip, mask);
            Ok(Subnet::new(ip, mask))
        }
    }

    pub fn aggregate(subnets: &[Subnet]) -> Result<Subnet, &'static str> {
        if subnets.is_empty() {
            return Err("Subnet list is empty");
        }
        if subnets.len() == 1 {
            info!("Single subnet provided: {:?}", subnets[0]);
            return Ok(subnets[0]);
        }

        let common_prefix = Self::calculate_common_prefix(subnets);

        let common_bits = Self::count_common_bits(subnets);
        info!("Common prefix length: {}", common_bits);

        // Zero out the bits that are not common
        let new_mask = !0 << (32 - common_bits);

        let aggregated_network = Ipv4Addr::from(common_prefix & new_mask);
        info!(
            "Aggregated network: IP = {}, Mask = {}",
            aggregated_network, common_bits
        );
        Ok(Subnet::new(aggregated_network, common_bits))
    }

    fn calculate_common_prefix(subnets: &[Subnet]) -> u32 {
        subnets
            .iter()
            .map(|subnet| u32::from(subnet.ip) & Self::mask_to_u32(subnet.mask))
            .fold(u32::MAX, |acc, masked_ip| acc & masked_ip)
    }

    pub fn count_common_bits(subnets: &[Subnet]) -> u32 {
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

    pub fn broadcast(&self) -> Ipv4Addr {
        let ip_u32 = u32::from(self.ip);
        let wildcard = !Self::mask_to_u32(self.mask);
        Ipv4Addr::from(ip_u32 | wildcard)
    }

    pub fn netmask(&self) -> Ipv4Addr {
        Ipv4Addr::from(Self::mask_to_u32(self.mask))
    }

    pub fn wildcard(&self) -> Ipv4Addr {
        Ipv4Addr::from(!Self::mask_to_u32(self.mask))
    }

    pub fn class(&self) -> char {
        match self.ip.octets()[0] {
            0..=127 => 'A',
            128..=191 => 'B',
            192..=223 => 'C',
            224..=239 => 'D',
            240..=255 => 'E',
        }
    }

    pub fn hosts(&self) -> u32 {
        2u32.pow(32 - self.mask) - 2
    }

    pub fn default_mask(ip: Ipv4Addr) -> u32 {
        match ip.octets()[0] {
            0..=127 => 8,    // Class A
            128..=191 => 16, // Class B
            192..=223 => 24, // Class C
            _ => 24,         // Fallback to /24 for other cases
        }
    }

    pub fn mask_to_u32(mask: u32) -> u32 {
        !0 << (32 - mask)
    }
}

impl std::fmt::Display for Subnet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.ip, self.mask)
    }
}
