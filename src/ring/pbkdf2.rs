// https://github.com/briansmith/ring

#[cfg(test)]
mod tests {
    //use super::*;

    use std::num::NonZeroU32;
    use ring::digest::SHA256_OUTPUT_LEN;
    use ring::digest::SHA512_OUTPUT_LEN;
    use ring::pbkdf2;
    //use ring::pbkdf2::Algorithm;
    //use ring::pbkdf2::PBKDF2_HMAC_SHA1;
    use ring::pbkdf2::PBKDF2_HMAC_SHA256;
    //use ring::pbkdf2::PBKDF2_HMAC_SHA384;
    use ring::pbkdf2::PBKDF2_HMAC_SHA512;


    #[test]
    fn run() {
        // Scenario 1 - PBKDF2_HMAC_SHA256
        const PBKDF2_HMAC_SHA256_ITERATIONS: u32 = 600_000; // number recommended by OWASP for PBKDF2 with SHA256

        // Prepare iterations, salt and secret
        let iterations = NonZeroU32::new(PBKDF2_HMAC_SHA256_ITERATIONS).unwrap();
        let salt = b"random salt"; // this should be randomly generated, using some user specific component and database specific component
        let secret = b"strong password"; // select a strong password
        println!("Secret/password value: {}", hex::encode(secret)); // don't print this in production

        // Derive the password hash and store
        let mut password_hash = [0u8; SHA256_OUTPUT_LEN]; // initialise out with zeros
        pbkdf2::derive(PBKDF2_HMAC_SHA256, iterations, salt, secret, &mut password_hash);
        println!("Password hash: {}", hex::encode(password_hash)); // don't print this in production

        // Verify whether or not a password matches the stored password hash
        pbkdf2::verify(PBKDF2_HMAC_SHA256, iterations, salt, secret, &password_hash).unwrap(); // success case
        //pbkdf2::verify(PBKDF2_HMAC_SHA256, iterations, salt, "wrong password".as_bytes(), &password_hash).unwrap(); // failure case


        // Scenario 2 - PBKDF2_HMAC_SHA512
        const PBKDF2_HMAC_SHA512_ITERATIONS: u32 = 210_000; // number recommended by OWASP for PBKDF2 with SHA512

        // Prepare iterations, salt and secret
        let iterations = NonZeroU32::new(PBKDF2_HMAC_SHA512_ITERATIONS).unwrap();
        let salt = b"random salt"; // this should be randomly generated, using some user specific component and database specific component
        let secret = b"strong password"; // select a strong password
        println!("Secret/password value: {}", hex::encode(secret)); // don't print this in production

        // Derive the password hash and store
        let mut password_hash = [0u8; SHA512_OUTPUT_LEN]; // initialise out with zeros
        pbkdf2::derive(PBKDF2_HMAC_SHA512, iterations, salt, secret, &mut password_hash);
        println!("Password hash: {}", hex::encode(password_hash)); // don't print this in production

        // Verify whether or not a password matches the stored password hash
        pbkdf2::verify(PBKDF2_HMAC_SHA512, iterations, salt, secret, &password_hash).unwrap(); // success case
        //pbkdf2::verify(PBKDF2_HMAC_SHA512, iterations, salt, "wrong password".as_bytes(), &password_hash).unwrap(); // failure case
    }

}





