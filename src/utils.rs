use std::net::Ipv4Addr;

pub fn subnet_mask(bits: u32) -> u32 {
    !0 << (32 - bits)
}

pub fn u32_to_dotted_decimal(ip: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        (ip >> 24) & 0xFF,
        (ip >> 16) & 0xFF,
        (ip >> 8) & 0xFF,
        ip & 0xFF
    )
}

pub fn default_mask(ip: Ipv4Addr) -> u32 {
    match ip.octets()[0] {
        0..=127 => 8,    // Class A
        128..=191 => 16, // Class B
        192..=223 => 24, // Class C
        _ => 24,         // Fallback to /24 for other cases
    }
}
