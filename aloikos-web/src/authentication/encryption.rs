use crate::config::CONFIG;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    XChaCha20Poly1305, XNonce, Key
};
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug)]
pub enum EncryptionError {
    EncryptionFailed(String),
    DecryptionFailed(String),
    InvalidKey(String),
    InvalidData(String),
}

impl std::fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionError::EncryptionFailed(e) => write!(f, "Encryption failed: {}", e),
            EncryptionError::DecryptionFailed(e) => write!(f, "Decryption failed: {}", e),
            EncryptionError::InvalidKey(e) => write!(f, "Invalid key: {}", e),
            EncryptionError::InvalidData(e) => write!(f, "Invalid data: {}", e),
        }
    }
}

impl std::error::Error for EncryptionError {}

// Derive a 32-byte key from the config string
fn derive_key() -> Result<Key, EncryptionError> {
    use sha2::{Digest, Sha256};
    
    let key_material = CONFIG.otp_encryption_key.as_bytes();
    if key_material.is_empty() {
        return Err(EncryptionError::InvalidKey("Encryption key is empty".to_string()));
    }
    
    // Hash the key material to get exactly 32 bytes
    let mut hasher = Sha256::new();
    hasher.update(key_material);
    let hashed = hasher.finalize();
    
    Ok(*Key::from_slice(&hashed))
}

pub fn encrypt_string(text: &str) -> Result<String, EncryptionError> {
    let key = derive_key()?;
    let cipher = XChaCha20Poly1305::new(&key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    
    let ciphertext = cipher
        .encrypt(&nonce, text.as_bytes())
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;
    
    // Combine nonce + ciphertext and encode as base64
    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);
    
    Ok(general_purpose::STANDARD.encode(combined))
}

pub fn decrypt_string(encrypted: &str) -> Result<String, EncryptionError> {
    let key = derive_key()?;
    let cipher = XChaCha20Poly1305::new(&key);
    
    // Decode from base64
    let combined = general_purpose::STANDARD
        .decode(encrypted)
        .map_err(|e| EncryptionError::InvalidData(format!("Invalid base64: {}", e)))?;
    
    // Split nonce and ciphertext
    if combined.len() < 24 {
        return Err(EncryptionError::InvalidData("Data too short".to_string()));
    }
    
    let (nonce_bytes, ciphertext) = combined.split_at(24);
    let nonce = XNonce::from_slice(nonce_bytes);
    
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;
    
    String::from_utf8(plaintext)
        .map_err(|e| EncryptionError::DecryptionFailed(format!("Invalid UTF-8: {}", e)))
}