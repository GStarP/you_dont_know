use chacha20poly1305::{aead::Aead, KeyInit, XChaCha20Poly1305, XNonce};

// ==========
// Constant
// ==========
static NONCE_BYTES: [u8; 24] = [7u8; 24];

// ==========
// Source
// ==========
pub fn encrypt_bytes(bytes: &Vec<u8>, password: &str) -> Vec<u8> {
    let cipher =
        XChaCha20Poly1305::new_from_slice(&password_to_key(password)).expect("invalid password");
    let nonce: XNonce = XNonce::from(NONCE_BYTES);
    return cipher
        .encrypt(&nonce, bytes.as_ref())
        .expect("encrypt failed");
}

pub fn decrypt_bytes(bytes: &Vec<u8>, password: &str) -> Vec<u8> {
    let cipher =
        XChaCha20Poly1305::new_from_slice(&password_to_key(password)).expect("invalid password");
    let nonce: XNonce = XNonce::from(NONCE_BYTES);
    return cipher
        .decrypt(&nonce, bytes.as_ref())
        .expect("decrypt failed");
}

/**
 * convert password to 32-bytes key
 */
fn password_to_key(password: &str) -> [u8; 32] {
    let mut res = [0u8; 32];

    let password = password.as_bytes();
    let len = password.len();
    let len = if len > 32 { 32 } else { len };

    for i in 0..len {
        res[i] = password[i];
    }

    res
}

// ==========
// Test
// ==========
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn encrypt_txt() {
        let filename = "assets/test.txt";
        let password = "123abc";

        let raw_bytes = fs::read(filename).unwrap();
        let raw_text = String::from_utf8_lossy(&raw_bytes);
        println!("before encrypt: {}", raw_text);

        let encrypted_bytes = encrypt_bytes(&raw_bytes, password);
        let encrypted_text = String::from_utf8_lossy(&encrypted_bytes);
        println!("after encrypt : {}", encrypted_text);

        assert_ne!(raw_text, encrypted_text);

        let decrypted_bytes = decrypt_bytes(&encrypted_bytes, password);
        let decrypted_text = String::from_utf8_lossy(&decrypted_bytes);
        println!("after decrypt : {}", decrypted_text);

        assert_eq!(raw_text, decrypted_text);
    }

    #[test]
    fn encrypt_image() {
        let filename = "assets/pokemon.jfif";
        let password = "i_love_pokemon";

        let raw_bytes = fs::read(filename).unwrap();
        let encrypted_bytes = encrypt_bytes(&raw_bytes, password);
        let encrypt_filename = "assets/pokemon-enc.jfif";
        fs::write(encrypt_filename, &encrypted_bytes);

        let decrypted_bytes = decrypt_bytes(&encrypted_bytes, password);
        let decrypt_filename = "assets/pokemon-dec.jfif";
        fs::write(decrypt_filename, &decrypted_bytes);
    }
}
