

// fn char_to_num(c: u8) -> u8 {
//     match c {
//         b'a'..=b'f' => c - b'a' + 10,
//         b'0'..=b'9' => c - b'0',
//         _ => panic!("invalid hex character"),
//     }
// }
//
// fn num_to_char(n: u8) -> u8 {
//     match n {
//         10..=15 => n - 10 + b'a',
//         0..=9 => n + b'0',
//         _ => panic!("invalid hex character"),
//     }
// }

// fn hex_to_bytes(hex: &str) -> Vec<u8> {
//     if hex.len() % 2 != 0 {
//         panic!("hex str length is not even")
//     }
//
//     let hex_upper = hex.to_lowercase();
//     let chars  = hex_upper.as_bytes();
//     let mut bytes = Vec::new();
//
//     // for each pair of characters
//     for i in (0..chars.len()).step_by(2) {
//
//         // map each character to its numeric value
//         // then multiply first value by 16 and add to second value
//         let result = char_to_num(chars[i]) * 16 + char_to_num(chars[i + 1]);
//
//         bytes.push(result);
//     }
//
//     bytes
//
//     //hex::decode(hex).unwrap()
// }

// fn bytes_to_hex(bytes: &[u8]) -> String {
//     let mut hex = String::new();
//
//     for b in bytes.iter() {
//         let first = b / 16;
//         let second = b % 16;
//         hex.push(num_to_char(first) as char);
//         hex.push(num_to_char(second) as char);
//     }
//
//     hex
//     //hex::encode(bytes.as_ref())
// }

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex::decode(hex).unwrap()
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes.as_ref())
}

use base64::{Engine as _, alphabet, engine::{self, general_purpose}};

fn base64_to_bytes(base64: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(base64).unwrap()
}

fn bytes_to_base64(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_challenge1() {
        let input_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let input_base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        // convert hex to base64
        let result_base64 = bytes_to_base64(&hex_to_bytes(input_hex));
        assert_eq!(input_base64, result_base64);

        // convert base64 to hex
        let result_hex = bytes_to_hex(&base64_to_bytes(input_base64));
        assert_eq!(input_hex, result_hex)
    }

}





