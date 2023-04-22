// https://github.com/briansmith/ring

#[cfg(test)]
mod tests {
    //use super::*;

    use ring::rand;
    use ring::hmac;

    #[test]
    fn run() {
        // Create a secure random number generator
        let rng = rand::SystemRandom::new();

        // Generate the hmac signing key
        let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng).unwrap();

        // Create a message to be signed
        let msg = "This is the message to be signed";

        // Sign the message
        let tag = hmac::sign(&key, msg.as_bytes());

        // Verify the integrity of the message - success case
        hmac::verify(&key, msg.as_bytes(), tag.as_ref()).unwrap();

        // Verify the integrity of the message - failure case
        //hmac::verify(&key, "This is the message to be signed - but altered".as_bytes(), tag.as_ref()).unwrap();
    }

    #[test]
    fn run_multipart() {
        // Create a secure random number generator
        let rng = rand::SystemRandom::new();

        // Generate the hmac signing key
        let key = hmac::Key::generate(hmac::HMAC_SHA512, &rng).unwrap();

        // Create a message to be signed
        let msg = ["hello", ", ", "world"];

        // Sign the message
        let mut context = hmac::Context::with_key(&key);
        for str in &msg {
            context.update(str.as_bytes());
        }
        let tag = context.sign();

        // concat the message data
        let mut msg_data = Vec::<u8>::new();
        for str in &msg {
            msg_data.extend(str.as_bytes());
        }

        // Verify the integrity of the message - success case
        hmac::verify(&key, &msg_data.as_ref(), tag.as_ref()).unwrap();
    }
}





