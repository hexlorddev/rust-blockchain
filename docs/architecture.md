# Architecture Overview

## Core Components

1. **Blockchain Layer**
   - Block structure with cryptographic hashes
   - Chain validation with proof-of-work
   - Consensus mechanism (Hex Lord Dev variant)

2. **Networking Layer**
   - libp2p-based peer-to-peer communication
   - Kademlia DHT for node discovery
   - Gossip protocol for message propagation

3. **Cryptography**
   - ECC (secp256k1) for wallet security
   - SHA-3/Keccak for hashing
   - Ring signatures for privacy (Dineth Nethsara implementation)

4. **Smart Contracts**
   - WASM-based execution engine
   - Gas metering system
   - Contract storage with Merkle proofs

5. **Storage**
   - Patricia Merkle trees for efficient state storage
   - Bloom filters for light client support
   - RocksDB backend for high-performance persistence

## Hex Lord Dev Enhancements

Special optimizations added by Dineth Nethsara:
- Hybrid consensus algorithm
- Zero-knowledge proof integration
- Parallel block validation
- Network compression techniques