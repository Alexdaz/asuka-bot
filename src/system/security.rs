use std::{io::{Read, Write}, path::Path};
use std::fs::File;

use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use sha2::{Digest, Sha256};

use crate::system::console::a_print;

const ENCRYPTED_ENV_PATH: &str = ".env.ask";

struct EncryptedVar {
    nonce: [u8; 12],
    ciphertext: Vec<u8>,
}

fn generate_nonce() -> [u8; 12] 
{
    let mut nonce: [u8; 12] = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    return nonce;
}

fn generate_key() -> [u8; 32] 
{
    let mid: String = machine_uid::get().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(mid.as_bytes());

    let result = hasher.finalize();

    let mut key: [u8; 32] = [0u8; 32];
    key.copy_from_slice(&result[..32]);

    return key;
}

fn encrypt_value(value: &str, key: &[u8]) -> EncryptedVar 
{
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce_bytes: [u8; 12] = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext: Vec<u8> = cipher.encrypt(nonce, value.as_bytes()).expect("Error encrypting");

    return EncryptedVar {
        nonce: nonce_bytes,
        ciphertext,
    };
}

fn decrypt_value(var: &EncryptedVar, key: &[u8]) -> String 
{
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(&var.nonce);

    match cipher.decrypt(nonce, var.ciphertext.as_ref()) {
        Ok(plaintext) => match String::from_utf8(plaintext) {
            Ok(s) => s,
            Err(_) => String::new(),
        },
        Err(_) => String::new(),
    }
}

pub fn encrypt_env_var(var_name: &str) 
{
    let key: [u8; 32] = generate_key();
    let encrypted: EncryptedVar = encrypt_value(var_name, &key);

    let mut file: File = File::create(ENCRYPTED_ENV_PATH).expect("Failed to create file");
    file.write_all(&encrypted.nonce).expect("Error writing nonce");
    file.write_all(&encrypted.ciphertext).expect("Error writing ciphertext");

    a_print("API KEY encrypted and stored securely. 🔐");
}

pub fn token_exists() -> bool 
{
    return !Path::new(ENCRYPTED_ENV_PATH).exists();
}

pub fn decrypt() -> String 
{
    let key: [u8; 32] = generate_key();
    let mut file: File = File::open(ENCRYPTED_ENV_PATH).expect("File not found");

    let mut nonce: [u8; 12] = [0u8; 12];
    file.read_exact(&mut nonce).expect("Error reading nonce");

    let mut ciphertext: Vec<u8> = Vec::new();
    file.read_to_end(&mut ciphertext).expect("Error reading ciphertext");

    let var: EncryptedVar = EncryptedVar {
        nonce,
        ciphertext,
    };

    return decrypt_value(&var, &key);
}
