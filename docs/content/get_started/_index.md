+++
title = "Getting Started"
description = "This section provides everything you need to get started with smugglex, from installation to your first scan."
weight = 1
sort_by = "weight"

[extra]
+++

# Getting Started with Smugglex

Welcome to the smugglex documentation! This section will guide you through everything you need to know to start testing for HTTP Request Smuggling vulnerabilities.

## What You'll Learn

In this section, you'll find:

- **[Overview](/get_started/overview)**: Learn about HTTP Request Smuggling vulnerabilities, how they work, and why smugglex is the tool you need
- **[Installation](/get_started/installation)**: Step-by-step instructions to install smugglex on your system

## Quick Start

If you're eager to get started, here's the fastest way to begin:

### 1. Install Smugglex

```bash
cargo install smugglex
```

### 2. Run Your First Scan

```bash
smugglex https://example.com/
```

### 3. Review Results

Smugglex will automatically test for all major HTTP Request Smuggling attack types and report any vulnerabilities found.

## Learning Path

We recommend following this learning path:

1. **Understand the Basics**: Read the [Overview](/get_started/overview) to understand what HTTP Request Smuggling is and how smugglex detects it
2. **Install the Tool**: Follow the [Installation](/get_started/installation) guide to get smugglex up and running
3. **Run Tests**: Start with simple scans on systems you have permission to test
4. **Explore Options**: Experiment with different command-line options and configurations
5. **Analyze Results**: Learn to interpret scan results and understand vulnerability details

## Example Workflow

Here's a typical workflow for using smugglex:

```bash
# Basic scan
smugglex https://target.com/

# Scan with verbose output and save results
smugglex https://target.com/ -v -o results.json

# Scan with custom headers and timeout
smugglex https://target.com/ -H "Authorization: Bearer token" -t 15

# Scan multiple URLs from a file
cat urls.txt | smugglex -v

# Run specific checks only
smugglex https://target.com/ -c cl-te,te-cl

# Export vulnerable payloads for manual verification
smugglex https://target.com/ --export-payloads ./payloads
```

## Key Concepts

Before you dive in, here are some key concepts to understand:

### HTTP Request Smuggling

A technique that exploits discrepancies in how front-end and back-end servers parse HTTP requests. When servers disagree on request boundaries, attackers can "smuggle" malicious requests through security controls.

### Attack Types

Smugglex tests for multiple attack types:
- **CL.TE**: Content-Length vs Transfer-Encoding desynchronization
- **TE.CL**: Transfer-Encoding vs Content-Length desynchronization
- **TE.TE**: Transfer-Encoding obfuscation (60+ variations)
- **H2C**: HTTP/2 Cleartext smuggling
- **H2**: HTTP/2 protocol-level smuggling

### Timing-Based Detection

Smugglex uses sophisticated timing analysis to detect vulnerabilities. By comparing response times between normal requests and attack requests, the tool can reliably identify desynchronization issues.

## Prerequisites

To use smugglex effectively, you should have:

- Basic understanding of HTTP protocol
- Knowledge of web application security
- Authorization to test target systems
- Familiarity with command-line tools

## Security Notice

⚠️ **Important**: Smugglex is a powerful security testing tool designed for:

- Authorized penetration testing
- Security research in controlled environments
- Bug bounty hunting on in-scope targets
- Educational purposes in lab settings

**Never** use smugglex on systems you don't own or don't have explicit written permission to test. Unauthorized testing may be illegal and unethical.

## Getting Help

Need assistance?

- Use `smugglex --help` for command-line help
- Check the [GitHub repository](https://github.com/hahwul/smugglex) for examples
- Open an [issue](https://github.com/hahwul/smugglex/issues) if you find bugs
- Follow [@hahwul](https://twitter.com/hahwul) on Twitter for updates

## Contributing

Smugglex is open-source and welcomes contributions! If you want to help improve the tool:

- Report bugs and suggest features via [GitHub Issues](https://github.com/hahwul/smugglex/issues)
- Submit pull requests with improvements
- Share your findings and help improve detection methods
- Contribute documentation and examples

## Ready to Begin?

Start with the [Overview](/get_started/overview) to understand HTTP Request Smuggling, or jump straight to [Installation](/get_started/installation) if you're ready to install the tool.
