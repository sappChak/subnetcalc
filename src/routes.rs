use std::net::Ipv4Addr;

use crate::{
    errors::RouteError,
    utils::{default_mask, subnet_mask},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Route {
    pub ip: Ipv4Addr,
    pub prefix: u32, // CIDR notation (e.g., /24)
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.ip, self.prefix)
    }
}

impl std::str::FromStr for Route {
    type Err = RouteError;

    fn from_str(subnet: &str) -> Result<Self, Self::Err> {
        let (ip_str, mask_str) = subnet.split_once('/').unwrap_or((subnet, ""));
        let ip = Ipv4Addr::from_str(ip_str).map_err(|_| RouteError::InvalidIpFormat)?;
        let mask = if mask_str.is_empty() {
            default_mask(ip)
        } else {
            mask_str
                .parse::<u32>()
                .map_err(|_| RouteError::InvalidMaskFormat)?
        };
        Ok(Route::new(ip, mask))
    }
}

impl Route {
    pub fn new(ip: Ipv4Addr, prefix: u32) -> Self {
        Self { ip, prefix }
    }

    pub fn broadcast_address(&self) -> Ipv4Addr {
        let ip_u32 = u32::from(self.ip);
        let wildcard = !subnet_mask(self.prefix);
        Ipv4Addr::from(ip_u32 | wildcard)
    }

    pub fn netmask_address(&self) -> Ipv4Addr {
        Ipv4Addr::from(subnet_mask(self.prefix))
    }

    pub fn wildcard_address(&self) -> Ipv4Addr {
        Ipv4Addr::from(!subnet_mask(self.prefix))
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
        2u32.pow(32 - self.prefix) - 2
    }
}

pub fn aggregate_routes(routes: &[Route]) -> Result<Route, RouteError> {
    if routes.is_empty() {
        return Err(RouteError::EmptyNetworkList);
    }
    if routes.len() == 1 {
        return Ok(routes[0]);
    }
    let (bits, count) = common_bits(routes);
    Ok(Route::new(Ipv4Addr::from(bits), count))
}

pub fn common_bits(routes: &[Route]) -> (u32, u32) {
    let mut common = u32::MAX;
    for route in routes {
        common &= u32::from(route.ip);
    }

    let max_prefix = routes.iter().map(|route| route.prefix).max().unwrap();
    let common_bit_count = (0..32)
        .rev()
        .take_while(|i| {
            let mask: u32 = 1 << i;
            routes
                .iter()
                .all(|route| (common & mask) == (u32::from(route.ip) & mask))
        })
        .count() as u32;

    (common, common_bit_count.min(max_prefix))
}

pub fn determine_subnet_mask(
    mask: u32,
    required_subnets: u32,
    required_hosts: u32,
) -> Result<Ipv4Addr, RouteError> {
    if required_hosts == 0 || required_subnets == 0 {
        return Err(RouteError::InvalidHostsOrSubnets);
    }

    let host_bits = (required_hosts + 2).next_power_of_two().trailing_zeros();
    let subnet_bits = required_subnets.next_power_of_two().trailing_zeros();

    if mask < host_bits || subnet_bits > 32 - mask {
        return Err(RouteError::InsufficientBits);
    }

    let new_mask_prefix = mask + subnet_bits;
    let new_mask = subnet_mask(new_mask_prefix);

    Ok(Ipv4Addr::from(new_mask.to_be_bytes()))
}
