// https://github.com/briansmith/ring

#[cfg(test)]
mod tests {
    //use super::*;

    use ring::error::Unspecified;
    use ring::rand::SystemRandom;
    use ring::agreement::Algorithm;
    // use ring::agreement::ECDH_P256;
    // use ring::agreement::ECDH_P384;
    use ring::agreement::X25519;
    use ring::agreement::EphemeralPrivateKey;
    use ring::agreement::PublicKey;
    use ring::agreement::UnparsedPublicKey;
    use ring::agreement::agree_ephemeral;

    #[test]
    fn run() -> Result<(), Unspecified> {
        // Use a rand::SystemRandom as the source of entropy
        let rng = SystemRandom::new();

        // Select a key agreement algorithm. All agreement algorithms follow the same flow
        let alg: &Algorithm = &X25519;

        // Generate a private key and public key
        let my_private_key: EphemeralPrivateKey = EphemeralPrivateKey::generate(alg, &rng)?;
        let my_public_key: PublicKey = my_private_key.compute_public_key()?;
        // The EphemeralPrivateKey doesn't allow us to directly access the private key as designed
        println!("my_public_key = {}", hex::encode(my_public_key.as_ref()));

        // Send our public key to the peer here

        // Simulate receiving a public key from the peer
        let peer_public_key: PublicKey = {
            let peer_private_key = EphemeralPrivateKey::generate(alg, &rng)?;
            peer_private_key.compute_public_key()?
        };
        println!("peer_public_key = {}", hex::encode(peer_public_key.as_ref()));

        // The peer public key needs to be parsed before use so wrap it creating as an instance of UnparsedPublicKey
        let peer_public_key = UnparsedPublicKey::new(alg, peer_public_key);

        // run ECDH to agree on a shared secret
        agree_ephemeral(my_private_key,
                        &peer_public_key,
                        Unspecified, // error to return on failure
                        |shared_secret: &[u8]| { // the result of the key agreement is passed to this lambda
                            println!("shared_secret = {}", hex::encode(shared_secret.as_ref())); // don't print this in production

                            // As recommended in RFC 7748 we should apply a KDF on the key material here before using in a real application
                            // We can return the derived key from the kdf here, otherwise we just return () if the key isn't needed outside this scope
                            Ok(())
                        },
        )
    }


    use std::thread;
    use std::net::{TcpListener, TcpStream};
    use std::io::{Read, Write};

    #[test]
    fn run_over_tcp() {
        let server = thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
            let mut stream = listener.accept().unwrap().0;

            ecdh_x25519("server", &mut stream).unwrap();
        });

        let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        ecdh_x25519("client", &mut stream).unwrap();

        server.join().unwrap();
    }

    fn ecdh_x25519(actor: &str, stream: &mut TcpStream) -> Result<(), Unspecified> {
        // Use a rand::SystemRandom as the source of entropy
        let rng = SystemRandom::new();

        // Select a key agreement algorithm. All agreement algorithms follow the same flow
        let alg: &Algorithm = &X25519;

        // Generate a private key and public key
        let my_private_key: EphemeralPrivateKey = EphemeralPrivateKey::generate(alg, &rng)?;
        let my_public_key: PublicKey = my_private_key.compute_public_key()?;
        // The EphemeralPrivateKey doesn't allow us to directly access the private key as designed
        println!("{}_public_key = {}", actor, hex::encode(my_public_key.as_ref()));

        // Send our public key to the peer here
        stream.write_all(my_public_key.as_ref()).unwrap();

        // Simulate receiving a public key from the peer
        let mut peer_public_key = [0u8; 32];
        stream.read_exact(&mut peer_public_key).unwrap();
        println!("{}_peer_public_key = {}", actor, hex::encode(peer_public_key.as_ref()));

        // The peer public key needs to be parsed before use so wrap it creating as an instance of UnparsedPublicKey
        let peer_public_key = UnparsedPublicKey::new(alg, peer_public_key);

        // run ECDH to agree on a shared secret
        agree_ephemeral(my_private_key,
                        &peer_public_key,
                        Unspecified, // error to return on failure
                        |shared_secret: &[u8]| { // the result of the key agreement is passed to this lambda
                            println!("{}_shared_secret = {}", actor, hex::encode(shared_secret.as_ref())); // don't print this in production

                            // As recommended in RFC 7748 we should apply a KDF on the key material here before using in a real application
                            // We can return the derived key from the kdf here, otherwise we just return () if the key isn't needed outside this scope
                            Ok(())
                        },
        )
    }

}





