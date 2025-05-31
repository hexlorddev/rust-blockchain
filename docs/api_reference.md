# API Reference

## Core Blockchain API

```rust
// Create a new blockchain
let mut blockchain = Blockchain::new();

// Add a block
blockchain.add_block("Block data".to_string());

// Validate chain
blockchain.is_valid();
```

## Cryptography API

```rust
// Generate keypair
let (private, public) = Crypto::generate_keypair();

// Sign message
let signature = Crypto::sign_message(&private, b"message");

// Verify signature
Crypto::verify_signature(&public, b"message", &signature);
```

## Networking API

```rust
// Start P2P node
let mut network = P2PNetwork::new();

// Connect to peer
network.add_peer("/ip4/1.2.3.4/tcp/1234".parse().unwrap());

// Run network (async)
tokio::spawn(async move {
    network.run().await;
});
```

## Smart Contract API

```rust
// Deploy contract
let instance = contract_engine.deploy_contract(wasm_bytes);

// Execute contract
let result = contract_engine.execute_contract(
    &instance,
    "entry_point",
    vec![Value::I32(42)]
);
```

*Hex Lord Dev Note: All APIs are designed for maximum performance and security*