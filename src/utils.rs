use std::net::Ipv4Addr;

pub fn mask_to_u32(mask: u32) -> u32 {
    !0 << (32 - mask)
}

pub fn default_mask(ip: Ipv4Addr) -> u32 {
    match ip.octets()[0] {
        0..=127 => 8,    // Class A
        128..=191 => 16, // Class B
        192..=223 => 24, // Class C
        _ => 24,         // Fallback to /24 for other cases
    }
}
