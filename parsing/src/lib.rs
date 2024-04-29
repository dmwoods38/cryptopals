use std::str::FromStr;

pub fn hex_str_to_bytes(hex_str: String) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let hex_str = String::from_str("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        let hex_bytes = hex_str_to_bytes(hex_str);
        let ans: Vec<u8> = [73, 39, 109, 32, 107, 105, 108, 108, 105, 110, 103, 32, 121, 111, 117, 114, 32, 98, 114, 97, 105, 110, 32, 108, 105, 107, 101, 32, 97, 32, 112, 111, 105, 115, 111, 110, 111, 117, 115, 32, 109, 117, 115, 104, 114, 111, 111, 109].to_vec();
        assert_eq!(hex_bytes, ans);
    }
}
