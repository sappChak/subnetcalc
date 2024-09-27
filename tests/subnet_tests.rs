#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use subnetcalc::{aggregate_subnets, find_common_prefix_length, mask_to_u32, parse_subnet};

    #[test]
    fn test_parse_subnet_valid() {
        let result = parse_subnet("192.168.100.0/27").unwrap();
        assert_eq!(result, (Ipv4Addr::new(192, 168, 100, 0), 27));

        let result = parse_subnet("10.0.0.0/8").unwrap();
        assert_eq!(result, (Ipv4Addr::new(10, 0, 0, 0), 8));
    }

    #[test]
    fn test_parse_subnet_invalid_format() {
        let result = parse_subnet("192.168.100.0-27");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid subnet format");

        let result = parse_subnet("invalid/27");
        assert!(result.is_err());
    }

    #[test]
    fn test_aggregate_subnets() {
        let subnets = vec![
            (Ipv4Addr::new(192, 168, 100, 0), 27),
            (Ipv4Addr::new(192, 168, 100, 32), 27),
            (Ipv4Addr::new(192, 168, 100, 64), 26),
        ];

        let result = aggregate_subnets(&subnets).unwrap();
        assert_eq!(result, (Ipv4Addr::new(192, 168, 100, 0), 25));
    }

    #[test]
    fn test_aggregate_single_subnet() {
        let subnets = vec![(Ipv4Addr::new(192, 168, 100, 0), 27)];

        let result = aggregate_subnets(&subnets).unwrap();
        assert_eq!(result, (Ipv4Addr::new(192, 168, 100, 0), 27)); // Single subnet stays the same
    }

    #[test]
    fn test_aggregate_subnets_empty() {
        let subnets = vec![];

        let result = aggregate_subnets(&subnets);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Subnet list is empty");
    }

    #[test]
    fn test_find_common_prefix_length() {
        let subnets = vec![
            (Ipv4Addr::new(192, 168, 100, 0), 27),
            (Ipv4Addr::new(192, 168, 100, 32), 27),
            (Ipv4Addr::new(192, 168, 100, 64), 26),
        ];

        let result = find_common_prefix_length(&subnets);
        assert_eq!(result, 25);
    }

    #[test]
    fn test_mask_to_u32() {
        assert_eq!(mask_to_u32(24), 0xFFFFFF00); // /24 should give a mask of 255.255.255.0
        assert_eq!(mask_to_u32(27), 0xFFFFFFE0); // /27 should give a mask of 255.255.255.224
    }
}
