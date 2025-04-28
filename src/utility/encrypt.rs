use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::RngCore; // AES-GCM cipher
use std::fs;
use std::path::Path;

const NONCE_SIZE: usize = 12; // 96-bit nonce size

pub fn generate_key(path: &Path) {
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    fs::write(path, &key_bytes).expect("Failed to write encryption key");
}

pub fn load_key(path: &Path) -> Aes256Gcm {
    let key_bytes = fs::read(path).expect("Failed to read encryption key");
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    Aes256Gcm::new(key)
}

pub fn load_secret_key() -> Aes256Gcm {
    let arc_key = Path::new(".arc/secret.key");
    let base_key = Path::new("secret.key");
    if arc_key.exists() {
        println!("Loaded secret key from .arc/secret.key");
        return load_key(arc_key);
    } else if base_key.exists() {
        println!("Loaded secret key from ./secret.key");
        return load_key(base_key);
    } else {
        panic!("No secret key found. Expected .arc/secret.key or ./secret.key");
    }
}

pub fn encrypt(cipher: &Aes256Gcm, data: &[u8]) -> Vec<u8> {
    let mut nonce = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce);
    let nonce_obj = Nonce::from_slice(&nonce);

    let ciphertext = cipher.encrypt(nonce_obj, data).expect("Encryption failed");

    // Prepend nonce to ciphertext
    let mut result = nonce.to_vec();
    result.extend(ciphertext);
    result
}

pub fn decrypt(cipher: &Aes256Gcm, data: &[u8]) -> Result<Vec<u8>, ()> {
    if data.len() < 12 {
        return Err(());
    }

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    match cipher.decrypt(nonce, ciphertext) {
        Ok(plaintext) => Ok(plaintext),
        Err(_) => Err(()),
    }
}

pub fn encrypt_to_file(cipher: &Aes256Gcm, data: &[u8], output_path: &Path) {
    let encrypted = encrypt(cipher, data);
    fs::write(output_path, encrypted).expect("Failed to write encrypted file");
}

pub fn decrypt_from_file(cipher: &Aes256Gcm, input_path: &Path) -> Vec<u8> {
    let encrypted = fs::read(input_path).expect("Failed to read encrypted file");
    decrypt(cipher, &encrypted).expect("Decryption failed")
}
