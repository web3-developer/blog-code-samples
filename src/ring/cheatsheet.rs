// https://github.com/briansmith/ring

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn run_digest() {
        use ring::digest;
        use ring::digest::Digest;
        use ring::digest::Context;
        use ring::digest::SHA256;

        // Calculate a hash using the digest function
        let digest: Digest = digest::digest(&SHA256, b"hello, world");
        println!("digest = {}", hex::encode(digest.as_ref()));

        // Calculate a hash using the Context struct
        let mut ctx = Context::new(&SHA256);
        ctx.update(b"hello, ");
        ctx.update(b"world");
        let digest = ctx.finish();
        println!("digest = {}", hex::encode(digest.as_ref()));
    }

    #[test]
    fn run_rand() {
        use ring::rand;
        use ring::rand::SecureRandom;
        use ring::rand::SystemRandom;
        use ring::rand::Random;

        const NUM_SIZE_BYTES: usize = 32;
        let sys_random = SystemRandom::new();

        // Generate random number using the SecureRandom::fill method
        let mut rand_bytes = [0u8; NUM_SIZE_BYTES];
        sys_random.fill(&mut rand_bytes).unwrap();
        println!("rand_bytes = {}", hex::encode(rand_bytes));

        // Generate random number using the rand::generate function
        let result : Random<[u8; NUM_SIZE_BYTES]> = rand::generate(&sys_random).unwrap();
        let rand_bytes = result.expose();
        println!("rand_bytes = {}", hex::encode(rand_bytes));
    }

    #[test]
    fn run_hmac() {
        use ring::rand;
        use ring::hmac;
        use ring::hmac::Key;
        use ring::hmac::Context;
        use ring::hmac::HMAC_SHA256;

        let sys_random = rand::SystemRandom::new();
        let key = Key::generate(HMAC_SHA256, &sys_random).unwrap();

        // Sign a message and then verify using the hmac::sign function
        let tag = hmac::sign(&key, b"hello, world");
        hmac::verify(&key, b"hello, world", tag.as_ref()).unwrap();
        println!("tag = {}", hex::encode(tag.as_ref()));

        // Sign a message and then verify using the Context struct
        let mut ctx = Context::with_key(&key);
        ctx.update(b"hello, ");
        ctx.update(b"world");
        let tag = ctx.sign();
        hmac::verify(&key, b"hello, world", tag.as_ref()).unwrap();
        println!("tag = {}", hex::encode(tag.as_ref()));
    }

    #[test]
    fn run_hkdf() {
        use ring::digest::SHA256_OUTPUT_LEN;
        use ring::hkdf::Salt;
        use ring::hkdf::Prk;
        use ring::hkdf::Okm;
        use ring::hkdf::Algorithm;
        use ring::hkdf::HKDF_SHA256;

        // Derive a single output key using Salt::extract and Prk::expand

        let input_key_material = b"secret key";
        println!("Input key material: {}", hex::encode(input_key_material)); // don't print this in production

        let salt = Salt::new(HKDF_SHA256, b"salt bytes");
        let pseudo_rand_key: Prk = salt.extract(input_key_material);
        let context_data = ["context field 1".as_bytes(), "context field 2".as_bytes()];
        let output_key_material: Okm<Algorithm> = pseudo_rand_key.expand(&context_data, HKDF_SHA256).unwrap();

        let mut result = [0u8; SHA256_OUTPUT_LEN];
        output_key_material.fill(&mut result).unwrap();
        println!("Derived output key material: {}", hex::encode(result)); // don't print this in production
    }

    #[test]
    fn run_pbkdf2() {
        use std::num::NonZeroU32;
        use ring::digest::SHA256_OUTPUT_LEN;
        use ring::pbkdf2;
        use ring::pbkdf2::PBKDF2_HMAC_SHA256;

        // Derive a password hash and verify using pbkdf2::derive and pbkdf2::verify

        let iterations = NonZeroU32::new(600_000).unwrap();
        let salt = b"random salt";
        let secret = b"strong password";
        println!("Secret/password value: {}", hex::encode(secret)); // don't print this in production

        let mut password_hash = [0u8; SHA256_OUTPUT_LEN];
        pbkdf2::derive(PBKDF2_HMAC_SHA256, iterations, salt, secret, &mut password_hash);
        pbkdf2::verify(PBKDF2_HMAC_SHA256, iterations, salt, secret, &password_hash).unwrap();
        println!("Password hash: {}", hex::encode(password_hash)); // don't print this in production
    }

    #[test]
    fn run_agreement() {

    }

    #[test]
    fn run_aead() {

    }

    #[test]
    fn run_signature() {

    }

}





