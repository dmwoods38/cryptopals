use std::str::FromStr;

use parsing::hex_str_to_bytes;

fn main() {
    let s1 = String::from_str("1c0111001f010100061a024b53535009181c").unwrap();
    let s2 = String::from_str("686974207468652062756c6c277320657965").unwrap();
    let res = fixed_xor(s1, s2);
    let ans = String::from_str("746865206b696420646f6e277420706c6179").unwrap();
    assert_eq!(res, ans);
}

fn fixed_xor(s1: String, s2: String) -> String {
    assert_eq!(s1.len(), s2.len());
    let mut xored_bytes: Vec<u8> = Vec::new();
    let s1_bytes = hex_str_to_bytes(s1);
    let s2_bytes = hex_str_to_bytes(s2);
    for i in 0..s1_bytes.len() {
       xored_bytes.push(s1_bytes[i]^s2_bytes[i]);
    } 
    xored_bytes.iter_mut().map(|b| format!("{b:x}")).collect()
}
