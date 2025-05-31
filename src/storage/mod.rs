use sha2::{Sha256, Digest};
use std::collections::HashMap;
use bloom::BloomFilter;
use std::error::Error;

#[derive(Default)]
pub struct MerkleTree {
    nodes: Vec<Vec<u8>>,
}

impl MerkleTree {
    pub fn new(leaves: Vec<Vec<u8>>) -> Self {
        let mut nodes = leaves;
        let mut level_size = nodes.len();
        
        while level_size > 1 {
            let mut new_level = Vec::new();
            for i in (0..level_size).step_by(2) {
                let left = &nodes[i];
                let right = if i + 1 < level_size {
                    &nodes[i + 1]
                } else {
                    left
                };
                
                let mut hasher = Sha256::new();
                hasher.update(left);
                hasher.update(right);
                new_level.push(hasher.finalize().to_vec());
            }
            nodes = new_level;
            level_size = nodes.len();
        }
        
        MerkleTree { nodes }
    }

    pub fn root(&self) -> Option<&Vec<u8>> {
        self.nodes.first()
    }
}

pub struct StateStorage {
    state: HashMap<Vec<u8>, Vec<u8>>,
    bloom_filter: BloomFilter,
}

impl StateStorage {
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        StateStorage {
            state: HashMap::new(),
            bloom_filter: BloomFilter::new(expected_items, false_positive_rate),
        }
    }

    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.bloom_filter.set(&key);
        self.state.insert(key, value);
    }

    pub fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        if self.bloom_filter.check(key) {
            self.state.get(key)
        } else {
            None
        }
    }
}