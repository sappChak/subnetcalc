# Subnet Analyzer

This Rust application calculates various subnet-related information, similar to the functionality provided by `ipcalc`. It can aggregate a list of subnets into a common network address and prefix length, and will include additional features for subnet calculations.

## Features

- **Implemented**:
  - Parse subnet strings into IP addresses and prefix lengths.
  - Aggregate multiple subnets into a common network address and prefix length.
  - Calculate the number of common prefix bits across all subnets.

- **To-Do**:
  - Calculate subnet details such as network address, broadcast address, and host range.
  - Display subnet information in various formats (binary, decimal, hexadecimal).
  - Convert between different subnet mask representations (e.g., CIDR notation, dotted decimal).
  - Provide detailed subnet information including the number of hosts, first and last host addresses, etc.

## Usage

### Running the Application

To aggregate IPs, use the following command:

```sh
cargo run aggregate "192.168.100.0/27" "192.168.100.32/27" "192.168.100.64/26"
```

To check subnet information such as broadcast and wildcard addresses, use:

```sh
cargo run info "192.168.100.0/27"
```

Note: If a prefix is not provided, the default based on the IP class will be applied.

## Tests

The application includes unit tests to verify the functionality of subnet parsing, aggregation, and utility functions. To run the tests, use the following command:

```sh
cargo test
```

## Subnet Aggregation Algorithm

The subnet aggregation algorithm combines multiple subnets into a single, larger subnet by determining the common bits shared among them. This can help optimize routing and reduce the size of routing tables.

### Steps of the Algorithm

1. **Bitwise AND Operation**: 
   - Perform a bitwise AND operation on all subnet IP addresses, masked by their respective subnet masks. This step identifies the common bits shared by all subnets.

2. **Counting Common Bits**: 
   - Count the number of common prefix bits across all subnets. Iterate over each bit position from the most significant to the least significant, checking if all subnets share the same bit value at that position.

3. **Constructing the Aggregated Network Address**: 
   - Once the common prefix length is determined, construct the aggregated network address by performing a bitwise AND operation between the common prefix and a mask that zeroes out the bits beyond the common prefix length.

### Example

Given the following subnets:
- `192.168.100.0/27`
- `192.168.100.32/27`
- `192.168.100.64/26`

#### Step 1: Perform Bitwise AND on All Subnets
```
11000000.10101000.01100100.00000000 (192.168.100.0/27)
11000000.10101000.01100100.00100000 (192.168.100.32/27)
11000000.10101000.01100100.01000000 (192.168.100.64/26)
```

#### Step 2: Count the Common Bits
```
11000000.10101000.01100100.00000000
11000000.10101000.01100100.00100000
11000000.10101000.01100100.01000000
---------------------------- (Common bits = 25)
```

#### Step 3: Obtain New Mask by Zeroing Out Uncommon Bits
```rust
let new_mask = !0 << (32 - common_bits);
// New mask: 11111111.11111111.11111111.10000000 (255.255.255.128)
```

#### Step 4: Construct the Aggregated Network Address
- The aggregated network address is `192.168.100.0/25`.
