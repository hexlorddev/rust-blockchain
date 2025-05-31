# Getting Started Guide

## Prerequisites

- Rust 1.70+ (recommended)
- Cargo
- Clang (for some cryptography dependencies)

## Installation

```bash
# Clone the repository
git clone https://github.com/dinethnethsara/rust-blockchain.git
cd rust-blockchain

# Build in release mode (recommended)
cargo build --release

# Run tests
cargo test

# Start the node
cargo run --release
```

## Configuration

Create `config.toml` in the project root:

```toml
[network]
port = 3030
bootnodes = ["/ip4/1.2.3.4/tcp/3030"]

[blockchain]
difficulty = 4
```

## Hex Lord Dev Tips

For optimal performance:
1. Use `--release` flag for production
2. Enable LTO in Cargo.toml
3. Set proper ulimits for network connections
4. Use hardware acceleration for cryptography