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

To run the application, use the following command:

```sh
cargo run aggregate "192.168.100.0/27" "192.168.100.32/27" "192.168.100.64/26"
```

### Example Output

The application will output the aggregated network address and prefix length. For the example command above, the output will be:

```
Aggregated network: 192.168.100.0/25
```

## Tests

The application includes unit tests to verify the functionality of subnet parsing, aggregation, and utility functions. To run the tests, use the following command:

```sh
cargo test
```

