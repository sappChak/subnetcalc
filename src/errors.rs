use std::error::Error;

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
