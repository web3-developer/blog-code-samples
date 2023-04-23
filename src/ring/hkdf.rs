// https://github.com/briansmith/ring

#[cfg(test)]
mod tests {
    //use super::*;

    use ring::digest::SHA256_OUTPUT_LEN;
    use ring::hkdf::{Algorithm, KeyType};
    use ring::hkdf::Okm;
    use ring::hkdf::Prk;
    use ring::hkdf::Salt;
    //use ring::hkdf::KeyType;
    //use ring::hkdf::HKDF_SHA1_FOR_LEGACY_USE_ONLY;
    use ring::hkdf::HKDF_SHA256;
    //use ring::hkdf::HKDF_SHA512;

    // impl KeyType for Algorithm {
    //     fn len(&self) -> usize {
    //         self.0.digest_algorithm().output_len
    //     }
    // }

    #[test]
    fn run() {
        let input_key_material = b"secret key";
        println!("Input key material: {}", hex::encode(input_key_material));

        // salt
        let salt = Salt::new(HKDF_SHA256, b"salt bytes");

        // Constructs a new Salt with the given value based on the given digest algorithm.
        // Constructing a Salt is relatively expensive so it is good to reuse a Salt object instead of re-constructing Salts with the same value.
        // prk
        let pseudo_rand_key: Prk = salt.extract(input_key_material);

        // The HKDF-Expand  operation.
        // Fails if (and only if) len is too large.
        // okm
        let context_data = &["context info".as_bytes()];
        let output_key_material: Okm<Algorithm> = pseudo_rand_key.expand(context_data, HKDF_SHA256).unwrap();

        let mut result = [0u8; SHA256_OUTPUT_LEN];
        output_key_material.fill(&mut result).unwrap();
        println!("Derived output key material: {}", hex::encode(result));

        // multiple calls to expand produce the same output
        // let output_key_material: Okm<Algorithm> = pseudo_rand_key.expand(context_data, HKDF_SHA256).unwrap();
        //
        // let mut result = [0u8; SHA256_OUTPUT_LEN];
        // output_key_material.fill(&mut result).unwrap();
        // println!("Derived output key material: {}", hex::encode(result));
    }

    #[test]
    fn run_expand_larger_len() {
        struct MyKeyType(usize);

        impl KeyType for MyKeyType {
            fn len(&self) -> usize {
                self.0
            }
        }

        let key_size = MyKeyType(SHA256_OUTPUT_LEN * 255);
        //let key_size = MyKeyType(SHA256_OUTPUT_LEN * 256); // fails

        let input_key_material = b"secret key";
        println!("Input key material: {}", hex::encode(input_key_material));

        // salt
        let salt = Salt::new(HKDF_SHA256, b"salt bytes");

        // Constructs a new Salt with the given value based on the given digest algorithm.
        // Constructing a Salt is relatively expensive so it is good to reuse a Salt object instead of re-constructing Salts with the same value.
        // prk
        let pseudo_rand_key: Prk = salt.extract(input_key_material);

        // The HKDF-Expand  operation.
        // Fails if (and only if) len is too large.
        // okm
        let context_data = &["context info".as_bytes()];
        let output_key_material: Okm<MyKeyType> = pseudo_rand_key.expand(context_data, key_size).unwrap();

        let mut result = [0u8; SHA256_OUTPUT_LEN * 255];
        output_key_material.fill(&mut result).unwrap();
        println!("Derived output key material: {}", hex::encode(result));

        // cannot call fill multiple times
        // let mut result = [0u8; SHA256_OUTPUT_LEN * 2];
        // output_key_material.fill(&mut result).unwrap();
        // println!("Derived output key material: {}", hex::encode(result));
    }
}





