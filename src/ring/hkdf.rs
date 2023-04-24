// https://github.com/briansmith/ring

#[cfg(test)]
mod tests {
    //use super::*;

    use ring::digest::SHA256_OUTPUT_LEN;
    //use ring::hkdf::HKDF_SHA1_FOR_LEGACY_USE_ONLY;
    use ring::hkdf::HKDF_SHA256;
    //use ring::hkdf::HKDF_SHA384;
    //use ring::hkdf::HKDF_SHA512;
    use ring::hkdf::KeyType;
    use ring::hkdf::Algorithm;
    use ring::hkdf::Salt;
    use ring::hkdf::Prk;
    use ring::hkdf::Okm;


    #[test]
    fn run() {
        // scenario 1 - generate single output key

        let input_key_material = b"secret key";
        println!("Input key material: {}", hex::encode(input_key_material));

        // Constructs a new Salt with the given value based on the given digest algorithm
        let salt = Salt::new(HKDF_SHA256, b"salt bytes");

        // The HKDF-Extract operation
        let pseudo_rand_key: Prk = salt.extract(input_key_material);

        // The HKDF-Expand operation
        let context_data = &["context one".as_bytes()];
        let output_key_material: Okm<Algorithm> = pseudo_rand_key.expand(context_data, HKDF_SHA256).unwrap();

        let mut result = [0u8; SHA256_OUTPUT_LEN];
        output_key_material.fill(&mut result).unwrap();
        println!("Derived output key material: {}", hex::encode(result));

        // second call with different context data produces a different output
        let context_data = &["context two".as_bytes()];
        let output_key_material: Okm<Algorithm> = pseudo_rand_key.expand(context_data, HKDF_SHA256).unwrap();

        let mut result = [0u8; SHA256_OUTPUT_LEN];
        output_key_material.fill(&mut result).unwrap();
        println!("Derived output key material: {}", hex::encode(result));


        // scenario 2 - generate multiple output keys

        const NUM_OF_KEYS: usize = 3;
        const OUTPUT_KEY_SIZE: usize = NUM_OF_KEYS * SHA256_OUTPUT_LEN;

        struct MyKeyType(usize);

        impl KeyType for MyKeyType {
            fn len(&self) -> usize {
                self.0
            }
        }

        // The HKDF-Expand operation
        let context_data = &["context one".as_bytes()];
        let output_key_material: Okm<MyKeyType> = pseudo_rand_key.expand(context_data, MyKeyType(OUTPUT_KEY_SIZE)).unwrap();

        let mut result = [0u8; OUTPUT_KEY_SIZE];
        output_key_material.fill(&mut result).unwrap();
        println!("Derived output key 1: {}", hex::encode(&result[0..32]));
        println!("Derived output key 2: {}", hex::encode(&result[32..64]));
        println!("Derived output key 3: {}", hex::encode(&result[64..96]));
    }

}





