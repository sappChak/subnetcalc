use std::net::Ipv4Addr;
use subnetcalc::subnet::Subnet;

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
    assert_eq!(result.unwrap_err(), "Invalid IP format");

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

#[test]
fn test_from_str_with_prefix() {
    let subnet_str = "192.168.1.0/24";
    let subnet = Subnet::from_str(subnet_str).expect("Failed to parse subnet");
    assert_eq!(subnet.ip, Ipv4Addr::new(192, 168, 1, 0));
    assert_eq!(subnet.mask, 24);
}

#[test]
fn test_from_str_without_prefix() {
    let subnet_str = "192.168.1.10";
    let subnet = Subnet::from_str(subnet_str).expect("Failed to parse subnet");
    assert_eq!(subnet.mask, 24);
    assert_eq!(subnet.ip, Ipv4Addr::new(192, 168, 1, 10)); // Defaulted to /24
}

#[test]
fn test_default_mask_class_a() {
    let ip = Ipv4Addr::new(10, 0, 0, 1);
    let mask = Subnet::default_mask(ip);
    assert_eq!(mask, 8);
}

#[test]
fn test_default_mask_class_b() {
    let ip = Ipv4Addr::new(172, 16, 0, 1);
    let mask = Subnet::default_mask(ip);
    assert_eq!(mask, 16);
}

#[test]
fn test_default_mask_class_c() {
    let ip = Ipv4Addr::new(192, 168, 1, 1);
    let mask = Subnet::default_mask(ip);
    assert_eq!(mask, 24);
}

#[test]
fn test_default_mask_other() {
    let ip = Ipv4Addr::new(224, 0, 0, 1);
    let mask = Subnet::default_mask(ip);
    assert_eq!(mask, 24); // Fallback
}

#[test]
fn test_broadcast() {
    let subnet = Subnet::new(Ipv4Addr::new(192, 168, 1, 0), 24);
    assert_eq!(subnet.broadcast(), Ipv4Addr::new(192, 168, 1, 255));
}

#[test]
fn test_netmask() {
    let subnet = Subnet::new(Ipv4Addr::new(192, 168, 1, 0), 24);
    assert_eq!(subnet.netmask(), Ipv4Addr::new(255, 255, 255, 0));
}

#[test]
fn test_wildcard() {
    let subnet = Subnet::new(Ipv4Addr::new(192, 168, 1, 0), 24);
    assert_eq!(subnet.wildcard(), Ipv4Addr::new(0, 0, 0, 255));
}

#[test]
fn test_class_a() {
    let subnet = Subnet::new(Ipv4Addr::new(10, 0, 0, 1), 8);
    assert_eq!(subnet.class(), 'A');
}

#[test]
fn test_class_b() {
    let subnet = Subnet::new(Ipv4Addr::new(172, 16, 0, 1), 16);
    assert_eq!(subnet.class(), 'B');
}

#[test]
fn test_class_c() {
    let subnet = Subnet::new(Ipv4Addr::new(192, 168, 0, 1), 24);
    assert_eq!(subnet.class(), 'C');
}

#[test]
fn test_class_d() {
    let subnet = Subnet::new(Ipv4Addr::new(224, 0, 0, 1), 4);
    assert_eq!(subnet.class(), 'D');
}

#[test]
fn test_class_e() {
    let subnet = Subnet::new(Ipv4Addr::new(240, 0, 0, 1), 4);
    assert_eq!(subnet.class(), 'E');
}

#[test]
fn test_hosts() {
    let subnet = Subnet::new(Ipv4Addr::new(192, 168, 1, 0), 24);
    assert_eq!(subnet.hosts(), 254);

    let subnet = Subnet::new(Ipv4Addr::new(10, 0, 0, 0), 8);
    assert_eq!(subnet.hosts(), 16_777_214);

    let subnet = Subnet::new(Ipv4Addr::new(172, 16, 0, 0), 16);
    assert_eq!(subnet.hosts(), 65_534);
}
