// https://github.com/briansmith/ring

#[cfg(test)]
mod tests {
    //use super::*;

    use ring::error::Unspecified;
    use ring::rand::SecureRandom;
    use ring::rand::SystemRandom;
    //use ring::aead::Algorithm;
    //use ring::aead::AES_128_GCM;
    use ring::aead::AES_256_GCM;
    //use ring::aead::CHACHA20_POLY1305;
    use ring::aead::UnboundKey;
    use ring::aead::BoundKey;
    use ring::aead::SealingKey;
    use ring::aead::OpeningKey;
    use ring::aead::Aad;
    //use ring::aead::Tag;
    use ring::aead::NonceSequence;
    use ring::aead::NONCE_LEN;
    use ring::aead::Nonce;

    // generally IV needs to be unique and unpredictable/random but not secret
    // generally nonce needs to be unique but not unpredictable or secret

    // For the nonces used for AES-GCM nonces must be unique and never used twice but they don't need to be unpredictable
    // max number of encryptions per key? Very large number 2^(12x8) - 1 encryption's if using counter nonce
    // if using random nonce then 2^30 encryptions using the same key
    struct CounterNonceSequence(u32);

    impl NonceSequence for CounterNonceSequence {
        // called once for each seal operation
        fn advance(&mut self) -> Result<Nonce, Unspecified> {
            self.0 += 1; // advance the counter
            let mut nonce_bytes = vec![0; NONCE_LEN];

            let bytes = self.0.to_be_bytes();
            nonce_bytes[8..].copy_from_slice(&bytes);
            println!("nonce_bytes = {}", hex::encode(&nonce_bytes));

            Nonce::try_assume_unique_for_key(&nonce_bytes)
        }
    }

    #[test]
    fn run_aes_256() -> Result<(), Unspecified> {
        // Create a new instance of SystemRandom to be used as the single source of entropy
        let rand = SystemRandom::new();

        // Generate a new symmetric encryption key
        let mut key_bytes = vec![0; AES_256_GCM.key_len()];
        rand.fill(&mut key_bytes)?;
        println!("key_bytes = {}", hex::encode(&key_bytes)); // don't print this in production code

        // Create a new AEAD key without a designated role or nonce sequence
        let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)?;

        // Create a new NonceSequence type which generates nonces
        // We can either store the state of the counter in the code or we can store it with the cypher text
        let nonce_sequence = CounterNonceSequence(0);

        // Create a new AEAD key for encrypting and signing ("sealing"), bound to a nonce sequence
        // The SealingKey can be used multiple times, each time a new nonce will be used
        let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);


        // This data will be authenticated but not encrypted
        //let associated_data = Aad::empty(); // is optional so can be empty
        let associated_data = Aad::from(b"additional public data");

        // Data to be encrypted
        let data = b"hello world";
        println!("data = {}", String::from_utf8(data.to_vec()).unwrap());

        // Create a mutable copy of the data that will be encrypted in place
        let mut in_out = data.clone();

        // Encrypt the data with AEAD using the AES_256_GCM algorithm
        let tag = sealing_key.seal_in_place_separate_tag(associated_data, &mut in_out)?;

        // Recreate the previously moved variables
        let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)?;
        let nonce_sequence = CounterNonceSequence(0);
        //let associated_data = Aad::empty(); // supplying the wrong data causes the decryption to fail
        let associated_data = Aad::from(b"additional public data");

        // Create a new AEAD key for decrypting and verifying the authentication tag
        let mut opening_key = OpeningKey::new(unbound_key, nonce_sequence);

        // Decrypt the data by passing in the associated data and the cypher text with the authentication tag appended
        let mut cypher_text_with_tag = [&in_out, tag.as_ref()].concat();
        let decrypted_data = opening_key.open_in_place( associated_data, &mut cypher_text_with_tag)?;
        println!("decrypted_data = {}", String::from_utf8(decrypted_data.to_vec()).unwrap());

        assert_eq!(data, decrypted_data);
        Ok(())
    }

}





