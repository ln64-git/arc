use crate::utility::encrypt;
use aes_gcm::Aes256Gcm;
use tempdir::TempDir;

#[test]
fn test_generate_key_and_load() {
    let temp_dir = TempDir::new("arc_test").unwrap();
    let key_path = temp_dir.path().join("test.key");

    encrypt::generate_key(&key_path);
    assert!(key_path.exists(), "Key file was not created");

    let cipher = encrypt::load_key(&key_path);
    let _cipher: Aes256Gcm = cipher;
}

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let temp_dir = TempDir::new("arc_test").unwrap();
    let key_path = temp_dir.path().join("test.key");
    encrypt::generate_key(&key_path);
    let cipher = encrypt::load_key(&key_path);

    let data = b"Hello Arc!";
    let encrypted = encrypt::encrypt(&cipher, data);
    let decrypted = encrypt::decrypt(&cipher, &encrypted).expect("Decryption failed");

    assert_eq!(data.to_vec(), decrypted);
}

#[test]
fn test_encrypt_to_file_and_decrypt_from_file() {
    let temp_dir = TempDir::new("arc_test").unwrap();
    let key_path = temp_dir.path().join("test.key");
    let output_path = temp_dir.path().join("output.enc");

    encrypt::generate_key(&key_path);
    let cipher = encrypt::load_key(&key_path);

    let data = b"Testing encryption file IO";
    encrypt::encrypt_to_file(&cipher, data, &output_path);

    let decrypted = encrypt::decrypt_from_file(&cipher, &output_path);
    assert_eq!(data.to_vec(), decrypted);
}
