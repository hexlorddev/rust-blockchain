mod crypto;
mod network;
mod contracts;
mod privacy;
mod storage;

use crypto::Crypto;
use network::P2PNetwork;
use contracts::SmartContractEngine;
use privacy::Privacy;
use storage::{MerkleTree, StateStorage};

#[tokio::main]
async fn main() {
    println!("Starting Advanced Rust Blockchain...");
    
    // Initialize cryptography
    let crypto = Crypto {};
    let (private_key, public_key) = crypto.generate_keypair();
    println!("Generated keypair: {:?} {:?}", private_key, public_key);
    
    // Initialize network
    let mut network = P2PNetwork::new().expect("Failed to initialize network");
    tokio::spawn(async move {
        network.run().await;
    });
    
    // Initialize smart contract engine
    let contract_engine = SmartContractEngine::new();
    
    // Initialize privacy features
    let privacy = Privacy {};
    
    // Initialize storage
    let mut state_storage = StateStorage::new(1000, 0.01);
    state_storage.insert(b"test_key".to_vec(), b"test_value".to_vec());
    
    // Create Merkle tree
    let leaves = vec![
        b"leaf1".to_vec(),
        b"leaf2".to_vec(),
        b"leaf3".to_vec(),
    ];
    let merkle_tree = MerkleTree::new(leaves);
    println!("Merkle root: {:?}", merkle_tree.root());
    
    println!("Advanced blockchain running!");
}