// https://github.com/web3-developer/fluent-hash

#[cfg(test)]
mod tests {
    //use super::*;

    use std::fs::File;
    use std::io::Write;
    use std::io::Error;
    use fluent_hash::Hashing::Sha1;
    use fluent_hash::Hashing::Sha256;
    use fluent_hash::Hashing::Sha384;
    use fluent_hash::Hashing::Sha512;
    use fluent_hash::Hashing::Sha512_256;
    use fluent_hash::Hashing;
    use fluent_hash::HashContext;
    use fluent_hash::Hash;


    #[test]
    fn run() -> Result<(), Error> {
        // Hashing a byte array
        let _hash: Hash = Sha1.hash(b"hello, world");

        // Hashing a byte vector
        let _hash: Hash = Sha256.hash_vec(b"hello, world".to_vec());

        // Hashing a string
        let _hash: Hash = Sha384.hash_str("hello, world");

        // Hashing a file
        let mut file = File::create("file.txt")?;
        file.write_all(b"hello, world")?;
        file.sync_all()?;
        let _hash: Hash = Sha512.hash_file("file.txt")?;

        // Hashing a file supports error handling
        let _error: Error = match Sha512.hash_file("notfound.txt") {
            Ok(_) => panic!("Expecting std::io::Error"),
            Err(e) => e
        };

        // Hash bytes using a HashContext
        let mut ctx: HashContext = Sha512_256.new_context();
        ctx.update(b"hello, world");
        ctx.update(b"more data");
        let hash = ctx.finish();

        // Format the hash.
        let bytes: &[u8] = hash.as_bytes();
        let bytes_vec: Vec<u8> = hash.to_vec();
        let hex: String = hash.to_hex();
        println!("bytes = {:?}", bytes);
        println!("bytes_vec = {:?}", bytes_vec);
        println!("hex = {}", hex);

        // Fluent interface supports method chaining
        let result = Hashing::Sha256
            .hash(b"hello, world")
            .to_hex();
        println!("result = {}", result);

        Ok(())
    }

}





