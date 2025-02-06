use bitcoin::hex::DisplayHex;

/// The ecb model is not safe, removed
fn old_encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
    use crypto::{aes::KeySize::KeySize256, blockmodes::NoPadding};

    let mut cipher = crypto::aes::ecb_encryptor(KeySize256, key, NoPadding);
    let mut out = vec![0; data.len()];
    let _ = cipher.encrypt(
        &mut RefReadBuffer::new(data),
        &mut RefWriteBuffer::new(&mut out),
        true,
    ); // ignore error of: InvalidLength, InvalidPadding.
    out
}

fn old_decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
    use crypto::{aes::KeySize::KeySize256, blockmodes::NoPadding};
    let mut cipher = crypto::aes::ecb_decryptor(KeySize256, key, NoPadding);
    let mut out = vec![0; data.len()];
    let _ = cipher.decrypt(
        &mut RefReadBuffer::new(data),
        &mut RefWriteBuffer::new(&mut out),
        true,
    );
    out
}

fn gcm_encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    use aes_gcm::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        Aes256Gcm, Key,
    };
    let key = Key::<Aes256Gcm>::from_slice(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let ciphered_data = cipher.encrypt(&nonce, data).expect("failed to encrypt");
    // combining nonce and encrypted data together
    // for storage purpose
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);
    encrypted_data
}

fn gcm_decrypt(key: &[u8], data: &[u8]) -> String {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Key, Nonce,
    };
    let key = Key::<Aes256Gcm>::from_slice(key);
    let (nonce_arr, ciphered_data) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);
    let cipher = Aes256Gcm::new(key);
    let plaintext = cipher
        .decrypt(nonce, ciphered_data)
        .expect("failed to decrypt data");
    String::from_utf8(plaintext).expect("failed to convert vector of bytes to string")
}

#[cfg(test)]
mod pre_test_encrypt {
    use super::*;
    use bitcoin::hex::DisplayHex;

    #[test]
    fn test_aes() {
        const TEXT: &str = "backendengineer.io";
        const KEY: &str = "thiskeystrmustbe32charlongtowork";

        let encrypted_data = gcm_encrypt(KEY.as_bytes(), TEXT.as_bytes());
        let original = gcm_decrypt(KEY.as_bytes(), &encrypted_data);
        println!("encrypted_data: {:?}", encrypted_data.to_lower_hex_string());
        println!("original: {:?}", original);

        let encrypted = old_encrypt(KEY.as_bytes(), TEXT.as_bytes());
        let source = old_decrypt(KEY.as_bytes(), &encrypted);
        println!("encrypted: {:?}", encrypted.to_lower_hex_string());
        println!("source: {:?}", source.to_lower_hex_string());
        println!("source: {:?}", String::from_utf8(source).expect("utf8"));

        assert!(false);
    }

    #[test]
    fn test_aes2() {
        use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};

        let key = GenericArray::from([0u8; 32]);
        let mut block = GenericArray::from([42u8; 16]);

        // Initialize cipher
        let cipher = aes::Aes256::new(&key);
        let block_copy = block.clone();

        println!("before: {:?}", block.to_lower_hex_string());
        // Encrypt block in-place
        cipher.encrypt_block(&mut block);
        println!("after: {:?}", block.to_lower_hex_string());

        // And decrypt it back
        cipher.decrypt_block(&mut block);
        assert_eq!(block, block_copy);

        assert!(false);
    }

    #[test]
    fn test_aes3() {
        const KEY: [u8; 32] = [0; 32];
        const DATA: [u8; 16] = [42; 16];
        println!("{}", DATA.to_lower_hex_string());

        let encrypted = aes_encrypt(&KEY, &DATA);
        println!("{}", encrypted.to_lower_hex_string());
        assert_eq!(encrypted.len(), DATA.len());

        let decrypted = aes_decrypt(&KEY, &encrypted.try_into().unwrap());
        println!("{}", decrypted.to_lower_hex_string());

        assert_eq!(decrypted, DATA);
    }
}

fn aes_encrypt(key: &[u8; 32], data: &[u8; 16]) -> Vec<u8> {
    use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};

    let key = GenericArray::from(*key);
    let mut block = GenericArray::from(*data);

    let cipher = aes::Aes256::new(&key);
    cipher.encrypt_block(&mut block);
    println!("encrypt: {:?}", block.to_lower_hex_string());

    block.to_vec()
}

fn aes_decrypt(key: &[u8; 32], data: &[u8; 16]) -> Vec<u8> {
    use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};

    let key = GenericArray::from(*key);
    let mut block = GenericArray::from(*data);

    let cipher = aes::Aes256::new(&key);
    cipher.decrypt_block(&mut block);
    println!("decrypt: {:?}", block.to_lower_hex_string());

    block.to_vec()
}
