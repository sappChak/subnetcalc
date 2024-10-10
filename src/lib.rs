pub mod subnet;

pub mod errors;

pub type Error = Box<dyn std::error::Error>;

pub type Result<T> = std::result::Result<T, Error>;
