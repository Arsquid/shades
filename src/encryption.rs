// src/encryption.rs

use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use hex::{decode, encode};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::error::Error;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn derive_key(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);

    key
}

pub fn encrypt_data(data: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let key = derive_key(password);
    let iv = rand::thread_rng().gen::<[u8; 16]>();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).map_err(|_| "Failed to create cipher.")?;

    let ciphertext = cipher.encrypt_vec(data.as_bytes());
    let mut encrypted_data = iv.to_vec();
    encrypted_data.extend(ciphertext);

    Ok(encode(&encrypted_data))
}

pub fn decrypt_data(encrypted_data: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let key = derive_key(password);
    let encrypted_data = decode(encrypted_data).map_err(|_| "Failed to decode encrypted data.")?;

    let (iv, ciphertext) = encrypted_data.split_at(16);
    let cipher = Aes256Cbc::new_from_slices(&key, iv)
        .map_err(|_| "Failed to create cipher for decryption.")?;

    let decrypted_data = cipher
        .decrypt_vec(ciphertext)
        .map_err(|_| "Failed to decrypt data.")?;

    String::from_utf8(decrypted_data)
        .map_err(|_| "Failed to convert decrypted data to string.".into())
}
