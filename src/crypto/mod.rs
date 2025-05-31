use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use sha3::Keccak256;
use ring::signature::{self, KeyPair};
use std::error::Error;

pub struct Crypto {}

impl Crypto {
    pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        (secret_key.secret_bytes().to_vec(), public_key.serialize().to_vec())
    }

    pub fn sign_message(secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(secret_key)?;
        let message_hash = Sha256::digest(message);
        let signature = secp.sign_ecdsa(&message_hash.into(), &secret_key);
        Ok(signature.serialize_der().to_vec())
    }

    pub fn verify_signature(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<bool, Box<dyn Error>> {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_slice(public_key)?;
        let message_hash = Sha256::digest(message);
        let signature = secp.ecdsa_signature_from_der(signature)?;
        Ok(secp.verify_ecdsa(&message_hash.into(), &signature, &public_key).is_ok())
    }

    pub fn keccak256_hash(data: &[u8]) -> Vec<u8> {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    pub fn ed25519_keypair() -> (Vec<u8>, Vec<u8>) {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        (key_pair.private_key().as_ref().to_vec(), key_pair.public_key().as_ref().to_vec())
    }
}