use log::info;
use std::error::Error;
use std::net::Ipv4Addr;

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

impl std::str::FromStr for Network {
    type Err = NetworkError;

    fn from_str(subnet: &str) -> Result<Self, Self::Err> {
        let (ip_str, mask_str) = subnet.split_once('/').unwrap_or((subnet, ""));
        let ip = Ipv4Addr::from_str(ip_str).map_err(|_| NetworkError::InvalidIpFormat)?;
        let mask = if mask_str.is_empty() {
            Self::default_mask(ip)
        } else {
            mask_str
                .parse::<u32>()
                .map_err(|_| NetworkError::InvalidMaskFormat)?
        };
        info!("Parsed network: IP = {}, Mask = {}", ip, mask);
        Ok(Network::new(ip, mask))
    }
}

#[derive(Debug, PartialEq)]
pub enum NetworkError {
    InvalidIpFormat,
    InvalidMaskFormat,
    EmptyNetworkList,
    InvalidHostsOrSubnets,
    InsufficientBits,
}

impl Error for NetworkError {}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkError::InvalidIpFormat => write!(f, "Invalid IP address format."),
            NetworkError::InvalidMaskFormat => write!(f, "Invalid subnet mask format."),
            NetworkError::EmptyNetworkList => write!(f, "The network list is empty."),
            NetworkError::InvalidHostsOrSubnets => {
                write!(f, "Invalid number of hosts or subnets provided.")
            }
            NetworkError::InsufficientBits => {
                write!(
                    f,
                    "Insufficient bits available for the required subnets or hosts."
                )
            }
        }
    }
}

impl Network {
    pub fn new(ip: Ipv4Addr, mask: u32) -> Self {
        Self { ip, mask }
    }

    pub fn aggregate_networks(networks: &[Network]) -> Result<Network, NetworkError> {
        if networks.is_empty() {
            return Err(NetworkError::EmptyNetworkList);
        }
        if networks.len() == 1 {
            info!("Single network provided: {:?}", networks[0]);
            return Ok(networks[0]);
        }

        let common_prefix = Self::find_common_prefix(networks);
        let common_bits = Self::count_common_bits(networks);
        info!("Common prefix length: {}", common_bits);

        let new_mask = Self::mask_to_u32(common_bits);
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
        mask: u32,
        required_subnets: u32,
        required_hosts: u32,
    ) -> Result<Ipv4Addr, NetworkError> {
        if required_hosts == 0 || required_subnets == 0 {
            return Err(NetworkError::InvalidHostsOrSubnets);
        }

        let host_bits = (required_hosts + 2).next_power_of_two().trailing_zeros();
        let subnet_bits = required_subnets.next_power_of_two().trailing_zeros();

        info!(
            "Required hosts: {}, Required subnets: {}, Host bits: {}, Subnet bits: {}",
            required_hosts, required_subnets, host_bits, subnet_bits,
        );

        if mask < host_bits || subnet_bits > 32 - mask {
            return Err(NetworkError::InsufficientBits);
        }

        let new_mask_prefix = mask + subnet_bits;
        info!("New mask prefix: {}", new_mask_prefix);
        let new_mask = Self::mask_to_u32(new_mask_prefix);

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
