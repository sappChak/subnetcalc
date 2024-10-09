use log::info;
use std::{net::Ipv4Addr, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Network {
    pub ip: Ipv4Addr,
    pub mask: u32, // CIDR notation (e.g., /24)
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.ip, self.mask)
    }
}

impl FromStr for Network {
    type Err = &'static str;

    fn from_str(subnet: &str) -> Result<Self, Self::Err> {
        if let Some((ip_str, mask_str)) = subnet.split_once('/') {
            let ip = Ipv4Addr::from_str(ip_str).map_err(|_| "Invalid IP format")?;
            let mask = mask_str.parse::<u32>().map_err(|_| "Invalid mask format")?;
            info!("Parsed network: IP = {}, Mask = {}", ip, mask);
            Ok(Network::new(ip, mask))
        } else {
            let ip = Ipv4Addr::from_str(subnet).map_err(|_| "Invalid IP format")?;
            let mask = Self::default_mask(ip);
            info!(
                "No prefix provided, using default mask: IP = {}, Mask = {}",
                ip, mask
            );
            Ok(Network::new(ip, mask))
        }
    }
}

impl Network {
    pub fn new(ip: Ipv4Addr, mask: u32) -> Self {
        Self { ip, mask }
    }

    pub fn aggregate_networks(networks: &[Network]) -> Result<Network, &'static str> {
        if networks.is_empty() {
            return Err("Network list is empty");
        }
        if networks.len() == 1 {
            info!("Single network provided: {:?}", networks[0]);
            return Ok(networks[0]);
        }

        let common_prefix = Self::find_common_prefix(networks);
        let common_bits = Self::count_common_bits(networks);
        info!("Common prefix length: {}", common_bits);

        let new_mask = !0 << (32 - common_bits);
        let aggregated_ip = Ipv4Addr::from(common_prefix & new_mask);
        info!(
            "Aggregated network: IP = {}, Mask = {}",
            aggregated_ip,
            Ipv4Addr::from(new_mask.to_be_bytes())
        );

        Ok(Network::new(aggregated_ip, common_bits))
    }

    fn find_common_prefix(networks: &[Network]) -> u32 {
        networks
            .iter()
            .map(|net| u32::from(net.ip) & Self::mask_to_u32(net.mask))
            .fold(u32::MAX, |acc, masked_ip| acc & masked_ip)
    }

    pub fn count_common_bits(networks: &[Network]) -> u32 {
        let first_ip = u32::from(networks[0].ip);
        (0..32)
            .rev()
            .take_while(|&i| {
                let mask = 1 << i;
                networks
                    .iter()
                    .all(|net| (first_ip & mask) == (u32::from(net.ip) & mask))
            })
            .count() as u32
    }

    pub fn determine_subnet_mask(
        network: &Network,
        required_hosts: u32,
        required_subnets: u32,
    ) -> Result<Ipv4Addr, String> {
        if required_hosts == 0 || required_subnets == 0 {
            return Err("Number of hosts and subnets must be greater than zero".to_string());
        }

        let host_bits = (required_hosts + 2).next_power_of_two().trailing_zeros();
        let subnet_bits = required_subnets.next_power_of_two().trailing_zeros();

        if network.mask < host_bits || subnet_bits > 32 - network.mask {
            return Err(
                "Not enough bits in the network for the required hosts or subnets".to_string(),
            );
        }

        let new_mask_prefix = network.mask + subnet_bits;
        let new_mask: u32 = !0 << (32 - new_mask_prefix);

        Ok(Ipv4Addr::from(new_mask.to_be_bytes()))
    }

    pub fn broadcast_address(&self) -> Ipv4Addr {
        let ip_u32 = u32::from(self.ip);
        let wildcard = !Self::mask_to_u32(self.mask);
        Ipv4Addr::from(ip_u32 | wildcard)
    }

    pub fn netmask_address(&self) -> Ipv4Addr {
        Ipv4Addr::from(Self::mask_to_u32(self.mask))
    }

    pub fn wildcard_address(&self) -> Ipv4Addr {
        Ipv4Addr::from(!Self::mask_to_u32(self.mask))
    }

    pub fn ip_class(&self) -> char {
        match self.ip.octets()[0] {
            0..=127 => 'A',
            128..=191 => 'B',
            192..=223 => 'C',
            224..=239 => 'D',
            240..=255 => 'E',
        }
    }

    pub fn available_hosts(&self) -> u32 {
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
