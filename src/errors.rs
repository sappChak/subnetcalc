use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum RouteError {
    InvalidIpFormat,
    InvalidMaskFormat,
    EmptyNetworkList,
    InvalidHostsOrSubnets,
    InsufficientBits,
}

impl Error for RouteError {}

impl std::fmt::Display for RouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouteError::InvalidIpFormat => write!(f, "Invalid IP address format."),
            RouteError::InvalidMaskFormat => write!(f, "Invalid subnet mask format."),
            RouteError::EmptyNetworkList => write!(f, "The network list is empty."),
            RouteError::InvalidHostsOrSubnets => {
                write!(f, "Invalid number of hosts or subnets provided.")
            }
            RouteError::InsufficientBits => {
                write!(
                    f,
                    "Insufficient bits available for the required subnets or hosts."
                )
            }
        }
    }
}
