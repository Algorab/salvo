use aead::generic_array::GenericArray;
use aead::{Aead, KeyInit};
use chacha20poly1305::ChaCha20Poly1305;

use super::CsrfCipher;

/// CcpCipher is a CSRF protection implementation that uses HMAC.
pub struct CcpCipher {
    aead_key: [u8; 32],
    token_size: usize,
}

impl CcpCipher {
    /// Given an HMAC key, return an `CcpCipher` instance.
    #[inline]
    pub fn new(aead_key: [u8; 32]) -> Self {
        Self {
            aead_key,
            token_size: 32,
        }
    }

    /// Sets the length of the token.
    #[inline]
    pub fn token_size(mut self, token_size: usize) -> Self {
        assert!(token_size >= 8, "length must be larger than 8");
        self.token_size = token_size;
        self
    }

    #[inline]
    fn aead(&self) -> ChaCha20Poly1305 {
        let key = GenericArray::clone_from_slice(&self.aead_key);
        ChaCha20Poly1305::new(&key)
    }
}

impl CsrfCipher for CcpCipher {
    fn verify(&self, token: &[u8], secret: &[u8]) -> bool {
        if token.len() < 8 || secret.len() < 20 {
            false
        } else {
            let nonce = GenericArray::from_slice(&secret[0..12]);
            let aead = self.aead();
            aead.decrypt(nonce, &secret[12..]).map(|p| p == token).unwrap_or(false)
        }
    }
    fn generate(&self) -> (Vec<u8>, Vec<u8>) {
        let token = self.random_bytes(self.token_size);
        let aead = self.aead();
        let mut secret = self.random_bytes(12);
        let nonce = GenericArray::from_slice(&secret);
        secret.append(&mut aead.encrypt(nonce, token.as_slice()).unwrap());
        (token, secret)
    }
}
