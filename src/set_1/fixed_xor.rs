use crate::set_1::convert_hex_to_base64::Hex;

fn fixed_xor(a: &str, b: &str) -> String {
    assert!(a.len() == b.len());
    let a = Hex::from_str(a).to_binary();
    let b = Hex::from_str(b).to_binary();
    let result: Vec<u8> = a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect();
    Hex::from_binary(&result).to_str()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";
        assert_eq!(fixed_xor(a, b), expected);
    }
}