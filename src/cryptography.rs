// src/cryptography.rs

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::Argon2;
use hex::{decode, encode};
use password_hash::rand_core::RngCore;
use std::error::Error;

pub fn derive_key(password: &str, salt: &[u8]) -> Result<Key<Aes256Gcm>, Box<dyn Error>> {
    let argon2 = Argon2::default();

    let mut key = Key::<Aes256Gcm>::default();
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|_| "Failed to derive key from password.")?;

    Ok(key)
}

pub fn encrypt_data(data: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let key = derive_key(password, &salt)?;

    let cipher = Aes256Gcm::new(&key);

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), data.as_bytes())
        .map_err(|_| "Encryption failed")?;

    let mut encrypted_data = Vec::new();
    encrypted_data.extend_from_slice(&salt);
    encrypted_data.extend_from_slice(&nonce);
    encrypted_data.extend_from_slice(&ciphertext);

    Ok(encode(&encrypted_data))
}

pub fn decrypt_data(encrypted_data: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let encrypted_data = decode(encrypted_data)?;

    let salt = &encrypted_data[0..16];
    let nonce = &encrypted_data[16..28];
    let ciphertext = &encrypted_data[28..];

    let key = derive_key(password, salt)?;

    let cipher = Aes256Gcm::new(&key);

    let decrypted_data = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| "Decryption failed")?;

    Ok(String::from_utf8(decrypted_data)?)
}
