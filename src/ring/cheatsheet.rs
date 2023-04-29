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

        // sha-256 using the digest function
        let sha256_digest: Digest = digest::digest(&SHA256, b"hello, world");
        println!("sha256_digest = {}", hex::encode(sha256_digest.as_ref()));

        // sha-256 using the Context struct
        let mut ctx = Context::new(&SHA256);
        ctx.update(b"hello, ");
        ctx.update(b"world");
        let sha256_digest = ctx.finish();
        println!("sha256_digest = {}", hex::encode(sha256_digest.as_ref()));
    }

    #[test]
    fn run_rand() {
        use ring::rand;
        use ring::rand::SecureRandom;
        use ring::rand::SystemRandom;
        use ring::rand::Random;

        const NUM_SIZE_BYTES: usize = 32;
        let sys_random = SystemRandom::new();

        // generate random number using the SecureRandom::fill method
        let mut rand_bytes = [0u8; NUM_SIZE_BYTES];
        sys_random.fill(&mut rand_bytes).unwrap();
        println!("rand_bytes = {}", hex::encode(rand_bytes));

        // generate random number using the rand::generate function
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

        let rng = rand::SystemRandom::new();
        let key = Key::generate(HMAC_SHA256, &rng).unwrap();

        // sign a message and then verify using the hmac::sign function
        let tag = hmac::sign(&key, b"hello, world");
        hmac::verify(&key, b"hello, world", tag.as_ref()).unwrap();
        println!("tag = {}", hex::encode(tag.as_ref()));

        // sign a message and then verify using the Context struct
        let mut ctx = Context::with_key(&key);
        ctx.update(b"hello, ");
        ctx.update(b"world");
        let tag = ctx.sign();
        hmac::verify(&key, b"hello, world", tag.as_ref()).unwrap();
        println!("tag = {}", hex::encode(tag.as_ref()));
    }

    #[test]
    fn run_hkdf() {

    }

    #[test]
    fn run_pbkdf2() {

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





