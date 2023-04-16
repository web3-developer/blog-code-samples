// https://github.com/briansmith/ring


// SystemRandom implements SecureRandom and supports creating a new instance with new
// Random is a struct that holds a T which implements RandomlyConstructable
// SecureRandom declares a fill method which supports filling a destination array with random bytes
// generate function takes a SecureRandom obj and returns a type that implements RandomlyConstructable

// Blog structure
// Background theory on random number generators and secure random number generators
// Explanation of the structs, traits and functions in the rand module and how they relate to each other
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

        // initialise a buffer with zeros
        let mut buffer = [0u8; 4];

        // fill the buffer with random bytes
        sys_random.fill(&mut buffer).unwrap();
        println!("{:?}", buffer);
        println!("{:?}", u32::from_be_bytes(buffer));

        // SystemRandom object and be reused through program
        sys_random.fill(&mut buffer).unwrap();
        println!("{:?}", buffer);
        println!("{:?}", u32::from_be_bytes(buffer));
    }

    #[test]
    fn generate_rand_numbers_using_generate_function() {
        // Create a secure random number generator
        let sys_random = SystemRandom::new();

        // generate the random number
        let result : Random<[u8; 4]> = rand::generate(&sys_random).unwrap();

        // get the value
        let rand_bytes = result.expose(); // can only be called once
        println!("{:?}", rand_bytes);
        println!("{:?}", u32::from_be_bytes(rand_bytes));

        // generate more random numbers using the same SystemRandom object
        let result : Random<[u8; 4]> = rand::generate(&sys_random).unwrap();
        let rand_bytes = result.expose();
        println!("{:?}", rand_bytes);
        println!("{:?}", u32::from_be_bytes(rand_bytes));
    }

}





