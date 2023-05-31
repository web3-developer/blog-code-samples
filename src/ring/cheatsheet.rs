// https://github.com/briansmith/ring

#[cfg(test)]
mod tests {
    //use super::*;

    use ring::error::Unspecified;

    #[test]
    fn run_digest() {
        use ring::digest;
        use ring::digest::Digest;
        use ring::digest::Context;
        use ring::digest::SHA256;

        // Calculate a hash using the digest function
        let _digest: Digest = digest::digest(&SHA256, b"hello, world");

        // Calculate a hash using the Context struct
        let mut ctx = Context::new(&SHA256);
        ctx.update(b"hello, ");
        ctx.update(b"world");

        let _digest = ctx.finish();
    }

    #[test]
    fn run_rand() -> Result<(), Unspecified> {
        use ring::rand;
        use ring::rand::SecureRandom;
        use ring::rand::SystemRandom;
        use ring::rand::Random;

        const NUM_SIZE_BYTES: usize = 32;
        let rand = SystemRandom::new();

        // Generate random number using the SecureRandom::fill method
        let mut rand_bytes = [0u8; NUM_SIZE_BYTES];
        rand.fill(&mut rand_bytes)?;

        // Generate random number using the rand::generate function
        let result: Random<[u8; NUM_SIZE_BYTES]> = rand::generate(&rand)?;
        let _rand_bytes = result.expose();

        Ok(())
    }

    #[test]
    fn run_hmac() -> Result<(), Unspecified> {
        use ring::rand::SystemRandom;
        use ring::hmac;
        use ring::hmac::Key;
        use ring::hmac::Context;
        use ring::hmac::HMAC_SHA256;

        let sys_random = SystemRandom::new();
        let key = Key::generate(HMAC_SHA256, &sys_random)?;

        // Sign a message and then verify using the hmac::sign function
        let tag = hmac::sign(&key, b"hello, world");
        hmac::verify(&key, b"hello, world", tag.as_ref())?;

        // Sign a message and then verify using the Context struct
        let mut ctx = Context::with_key(&key);
        ctx.update(b"hello, ");
        ctx.update(b"world");
        let tag = ctx.sign();

        hmac::verify(&key, b"hello, world", tag.as_ref())
    }

    #[test]
    fn run_hkdf() -> Result<(), Unspecified> {
        use ring::digest::SHA256_OUTPUT_LEN;
        use ring::hkdf::Salt;
        use ring::hkdf::Prk;
        use ring::hkdf::Okm;
        use ring::hkdf::Algorithm;
        use ring::hkdf::HKDF_SHA256;

        // Derive a single output key using Salt::extract and Prk::expand

        let input_key_material = b"secret key";

        let salt = Salt::new(HKDF_SHA256, b"salt bytes");
        let pseudo_rand_key: Prk = salt.extract(input_key_material);
        let context_data = ["context field 1".as_bytes(), "context field 2".as_bytes()];
        let output_key_material: Okm<Algorithm> = pseudo_rand_key.expand(&context_data, HKDF_SHA256)?;

        let mut result = [0u8; SHA256_OUTPUT_LEN];
        output_key_material.fill(&mut result)
    }

    #[test]
    fn run_pbkdf2() -> Result<(), Unspecified> {
        use std::num::NonZeroU32;
        use ring::digest::SHA256_OUTPUT_LEN;
        use ring::pbkdf2;
        use ring::pbkdf2::PBKDF2_HMAC_SHA256;

        // Derive a password hash and verify using pbkdf2::derive and pbkdf2::verify

        let iterations = NonZeroU32::new(600_000).unwrap();
        let salt = b"random salt";
        let secret = b"strong password";

        let mut password_hash = [0u8; SHA256_OUTPUT_LEN];
        pbkdf2::derive(PBKDF2_HMAC_SHA256, iterations, salt, secret, &mut password_hash);
        pbkdf2::verify(PBKDF2_HMAC_SHA256, iterations, salt, secret, &password_hash)
    }

    #[test]
    fn run_agreement() -> Result<(), Unspecified> {
        use ring::rand::SystemRandom;
        use ring::agreement;
        use ring::agreement::Algorithm;
        use ring::agreement::X25519;
        use ring::agreement::EphemeralPrivateKey;
        use ring::agreement::PublicKey;
        use ring::agreement::UnparsedPublicKey;

        // Derived a shared secret using ECDH

        let rng = SystemRandom::new();
        let alg: &Algorithm = &X25519;

        let my_private_key: EphemeralPrivateKey = EphemeralPrivateKey::generate(alg, &rng)?;
        let _my_public_key: PublicKey = my_private_key.compute_public_key()?;

        // Send our public key to the peer here

        let peer_public_key: PublicKey = { // Simulate receiving a public key from the peer
            let peer_private_key = EphemeralPrivateKey::generate(alg, &rng)?;
            peer_private_key.compute_public_key()?
        };

        let peer_public_key = UnparsedPublicKey::new(alg, peer_public_key);
        agreement::agree_ephemeral(my_private_key,
                        &peer_public_key,
                        Unspecified,
                        |_shared_secret: &[u8]| {
                            // use the shared secret
                            Ok(())
                        })
    }

    #[test]
    fn run_signature() -> Result<(), Unspecified> {
        use ring::rand::SystemRandom;
        use ring::signature::ED25519;
        use ring::signature::KeyPair;
        use ring::signature::Ed25519KeyPair;
        use ring::signature::UnparsedPublicKey;

        // Sign and verify a message using EdDSA

        let rand = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rand)?;
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).map_err(|_| Unspecified)?;

        const MESSAGE: &[u8] = b"hello, world";
        let sig = key_pair.sign(MESSAGE);

        let peer_public_key_bytes = key_pair.public_key().as_ref();
        let peer_public_key = UnparsedPublicKey::new(&ED25519, peer_public_key_bytes);
        peer_public_key.verify(MESSAGE, sig.as_ref())
    }

    #[test]
    fn run_aead() -> Result<(), Unspecified> {
        use ring::rand::SecureRandom;
        use ring::rand::SystemRandom;
        use ring::aead::AES_256_GCM;
        use ring::aead::UnboundKey;
        use ring::aead::BoundKey;
        use ring::aead::SealingKey;
        use ring::aead::OpeningKey;
        use ring::aead::Aad;
        use ring::aead::NonceSequence;
        use ring::aead::NONCE_LEN;
        use ring::aead::Nonce;

        // Always returns the same nonce value for simplicity, don't use for more than 1 sealing operation!
        struct SingleNonceSequence([u8; NONCE_LEN]);

        impl NonceSequence for SingleNonceSequence {
            fn advance(&mut self) -> Result<Nonce, Unspecified> {
                Nonce::try_assume_unique_for_key(&self.0)
            }
        }

        // Encrypt and decrypt some data using AES-GCM-256

        let rand = SystemRandom::new();

        let mut key_bytes = vec![0; AES_256_GCM.key_len()];
        rand.fill(&mut key_bytes)?;
        let mut nonce_value = [0; NONCE_LEN];
        rand.fill(&mut nonce_value)?;

        let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)?;
        let nonce_sequence = SingleNonceSequence(nonce_value);
        let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);

        let associated_data = Aad::from(b"additional public data");
        let data = b"hello world";
        let mut in_out = data.clone();
        let tag = sealing_key.seal_in_place_separate_tag(associated_data, &mut in_out)?;

        let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)?;
        let nonce_sequence = SingleNonceSequence(nonce_value);
        let mut opening_key = OpeningKey::new(unbound_key, nonce_sequence);

        let associated_data = Aad::from(b"additional public data");
        let mut cypher_text_with_tag = [&in_out, tag.as_ref()].concat();
        let decrypted_data = opening_key.open_in_place(associated_data, &mut cypher_text_with_tag)?;

        assert_eq!(data, decrypted_data);

        Ok(())
    }

}





