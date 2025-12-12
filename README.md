# smugglex

A powerful HTTP Request Smuggling testing tool written in Rust.

```
smugglex https://target.com/
11:27PM INF start scan to https://target.com/
11:29PM WRN smuggling found 2 vulnerability(ies)

=== TE.CL Vulnerability Details ===
Status: VULNERABLE
Payload Index: 0
Attack Response: Connection Timeout
Timing: Normal: 1279ms, Attack: 10000ms

=== TE.TE Vulnerability Details ===
Status: VULNERABLE
Payload Index: 11
Attack Response: Connection Timeout
Timing: Normal: 1263ms, Attack: 10000ms

11:29PM INF scan completed in 141.099 seconds
```

## Features

- **Multiple Attack Types**: Detect CL.TE, TE.CL, TE.TE, H2C, and H2 smuggling vulnerabilities
- **Extended Mutation Testing**: 30+ variations of Transfer-Encoding header obfuscations
- **HTTP/2 Support**: H2C and H2 protocol-level desync attack detection
- **Flexible Testing**: Custom headers, cookies, virtual hosts, and payload export
- **JSON Output**: Save scan results for further analysis
- **Pipeline Support**: Read URLs from stdin for integration with other tools

## Installation

### From crates.io

```bash
cargo install smugglex
```

### From source

```bash
git clone https://github.com/hahwul/smugglex
cd smugglex
cargo install --path .
```

Or build without installing:

```bash
cargo build --release
./target/release/smugglex
```

## Usage

Basic usage:

```bash
smugglex https://target.com
```

Advanced usage:

```bash
smugglex https://target.com -m POST -t 15 -v -o results.json
```

For detailed usage, options, and examples, please visit the [documentation site](https://smugglex.hahwul.com).

## Documentation

For comprehensive guides, attack type explanations, and advanced usage examples, please visit:

**📚 [https://smugglex.hahwul.com](https://smugglex.hahwul.com)**

## Security Warning

⚠️ **Warning**: This tool is designed for security testing and should only be used:
- On systems you own or have explicit permission to test
- In authorized penetration testing engagements
- For educational and research purposes in controlled environments

Unauthorized testing of web applications may be illegal in your jurisdiction.

## License

MIT
