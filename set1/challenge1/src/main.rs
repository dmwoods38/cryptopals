use std::str::FromStr;
use parsing::hex_str_to_bytes;

const BASE64_LOOKUP_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    let hex_str = String::from_str("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
    let hex_bytes = hex_str_to_bytes(hex_str);
    println!("{:?}", hex_bytes);
    let b64_bytes = bytes_to_b64_bytes(hex_bytes);
    println!("{:?}", b64_bytes);
    let b64_str = bytes_to_b64_str(b64_bytes);
    println!("{}", b64_str);
    assert_eq!(b64_str, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t" );
}
/*
fn hex_str_to_bytes(hex_str: String) -> Vec<u8> {
    if hex_str.len() % 2 != 0 {
        panic!("Invalid hex string");
    }
    let mut bytes: Vec<u8> = Vec::new();
    let hex_bytes = hex_str.chars().map(|c| c.to_digit(16).unwrap() as u8).collect::<Vec<u8>>();

    for i in 0..hex_bytes.len()/2 {
        bytes.push((hex_bytes[i*2] << 4) + hex_bytes[i*2+1]);
    }
    // take the string split it into 2 character slices then parse each item into a u8 
    bytes
}
*/

fn bytes_to_b64_bytes(bytes: Vec<u8>) -> Vec<u8>{
    let mut b64_bytes: Vec<u8> = Vec::new();

    for chunk in bytes.chunks(3) {
        b64_bytes.push(chunk[0] >> 2); //grab first 6 bits of first chunk
        if chunk.len() == 3 {
            b64_bytes.push((chunk[0] << 6 >> 2) + (chunk[1] >> 4)); //grab last two of first and first 4 of second
            b64_bytes.push((chunk[1] << 4 >> 2) + (chunk[2] >> 6)); //grab last four of second and first 2 of third
            b64_bytes.push(chunk[2] << 2 >> 2); //grab last 6th of third
        } else if chunk.len() == 2 {
            b64_bytes.push((chunk[0] << 6 >> 2) + (chunk[1] >> 4)); //grab last two of first and first 4 of second
            b64_bytes.push(chunk[1] << 4 >> 2);
        }
    }
    b64_bytes
}

//bytes should be u8 versions of b64 values
fn bytes_to_b64_str(bytes: Vec<u8>) -> String {
    let mut b64_str = String::new();
    let b64_lookup: Vec<char> = BASE64_LOOKUP_STR.chars().collect();
    let length = b64_lookup.len();
    for byte in bytes {
        if byte as usize >= length {
            panic!("Invalid byte: {}", byte);
        }
        b64_str.push(b64_lookup[byte as usize]);
    }
    let padding: &str;
    match b64_str.len() % 4 {
        3 => padding = "===",
        2 => padding = "==",
        1 => padding = "=",
        _ => padding = "",
    }
    b64_str.push_str(padding);
    b64_str
}
