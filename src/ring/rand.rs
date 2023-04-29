// https://github.com/briansmith/ring

// Blog structure

// Example 1
// Example 2


#[cfg(test)]
mod tests {
    //use super::*;

    use ring::rand;
    use ring::rand::SystemRandom;
    use ring::rand::Random;
    use ring::rand::SecureRandom;

    #[test]
    fn generate_rand_numbers_using_fill() {
        // Create a secure random number generator
        let sys_random = SystemRandom::new();

        // Initialise a buffer with zeros
        let mut buffer = [0u8; 4];

        // Fill the buffer with random bytes
        sys_random.fill(&mut buffer).unwrap();
        println!("{:?}", buffer);
        println!("{:?}", u32::from_be_bytes(buffer));

        // Generate the random number
        let result : Random<[u8; 4]> = rand::generate(&sys_random).unwrap();

        // Get the value
        let rand_bytes = result.expose(); // can only be called once
        println!("{:?}", rand_bytes);
        println!("{:?}", u32::from_be_bytes(rand_bytes));

        // Generate more random numbers using the same SystemRandom object
        let result : Random<[u8; 4]> = rand::generate(&sys_random).unwrap();
        let rand_bytes = result.expose();
        println!("{:?}", rand_bytes);
        println!("{:?}", u32::from_be_bytes(rand_bytes));
    }

}





