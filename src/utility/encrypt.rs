use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::RngCore;
use std::fs;
use std::path::Path;
use std::process;

const NONCE_SIZE: usize = 12; // 96-bit nonce size

pub fn generate_key(path: &Path) {
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    fs::write(path, &key_bytes).expect("Failed to write encryption key");
}

pub fn load_key(path: &Path) -> Aes256Gcm {
    let key_bytes = fs::read(path).expect(&format!("❌ Failed to read key at {:?}", path));
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    Aes256Gcm::new(key)
}

pub fn load_secret_key() -> Aes256Gcm {
    let arc_key = Path::new(".arc/secret.key");
    if arc_key.exists() {
        println!("🔑 Loaded secret key from .arc/secret.key");
        return load_key(arc_key);
    }
    display_missing_key_message_and_exit();
}

fn display_missing_key_message_and_exit() -> ! {
    println!("\n🚨 Missing Secret Key!");
    println!("ARC encryption requires a valid 256-bit secret key.");
    println!("Request access to the original ARC source and copy .arc/secret.key");
    panic!("Data length is less than the nonce size, decryption cannot proceed");
}

pub fn encrypt(cipher: &Aes256Gcm, data: &[u8]) -> Vec<u8> {
    let mut nonce = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce);
    let nonce_obj = Nonce::from_slice(&nonce);

    let ciphertext = cipher.encrypt(nonce_obj, data).expect("Encryption failed");

    let mut result = nonce.to_vec();
    result.extend(ciphertext);
    result
}

pub fn decrypt(cipher: &Aes256Gcm, data: &[u8]) -> Result<Vec<u8>, ()> {
    if data.len() < NONCE_SIZE {
        process::exit(1);
    }

    let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, ciphertext).map_err(|_| ())
}

pub fn encrypt_to_file(key: &Aes256Gcm, plaintext: &[u8], path: &Path) {
    if let Some(parent) = path.parent() {
        // ⚠️ If parent exists as a file, remove it before mkdir
        if parent.exists() && !parent.is_dir() {
            fs::remove_file(parent).expect("❌ Failed to remove conflicting file");
        }

        fs::create_dir_all(parent).expect("❌ Failed to create parent dirs");
    }

    let encrypted = encrypt(key, plaintext);
    fs::write(path, encrypted).expect("❌ Failed to write encrypted file");
}

pub fn decrypt_from_file(cipher: &Aes256Gcm, input_path: &Path) -> Vec<u8> {
    let encrypted = fs::read(input_path).expect("Failed to read encrypted file");
    match decrypt(cipher, &encrypted) {
        Ok(decrypted_data) => decrypted_data,
        Err(_) => {
            eprintln!("Decryption failed");
            process::exit(1);
        }
    }
}
