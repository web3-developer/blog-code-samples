// https://github.com/briansmith/ring

#[cfg(test)]
mod tests {
    //use super::*;

    use ring::digest::digest;
    use ring::digest::Digest;
    use ring::digest::Context;
    use ring::digest::SHA1_FOR_LEGACY_USE_ONLY;
    use ring::digest::SHA256;
    use ring::digest::SHA512;

    #[test]
    fn run() {
        // SHA-1 (deprecated) using digest function
        let sha1_digest: Digest = digest(&SHA1_FOR_LEGACY_USE_ONLY, b"hello, world");
        println!("{:?}", sha1_digest.algorithm());
        println!("{}", hex::encode(sha1_digest.as_ref()));

        // SHA-256 using digest function
        let sha256_digest: Digest = digest(&SHA256, b"hello, world");
        println!("{:?}", sha256_digest.algorithm());
        println!("{}", hex::encode(sha256_digest.as_ref()));

        // SHA-512 using digest function
        let sha512_digest: Digest = digest(&SHA512, b"hello, world");
        println!("{:?}", sha512_digest.algorithm());
        println!("{}", hex::encode(sha512_digest.as_ref()));


        // SHA-256 using Context struct
        let mut ctx = Context::new(&SHA256);
        ctx.update(b"hello");
        ctx.update(b", ");
        ctx.update(b"world");
        let ctx_digest = ctx.finish();

        println!("{:?}", ctx_digest.algorithm());
        println!("{}", hex::encode(ctx_digest.as_ref()));
    }
}





