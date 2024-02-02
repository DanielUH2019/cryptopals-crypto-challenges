// use std::collections::HashMap;
use phf::{phf_map, Map};

const HEX_MAP: Map<u8, [u8; 4]> = phf_map! {
    b'0' => [0, 0, 0, 0],
    b'1' => [0, 0, 0, 1],
    b'2' => [0, 0, 1, 0],
    b'3' => [0, 0, 1, 1],
    b'4' => [0, 1, 0, 0],
    b'5' => [0, 1, 0, 1],
    b'6' => [0, 1, 1, 0],
    b'7' => [0, 1, 1, 1],
    b'8' => [1, 0, 0, 0],
    b'9' => [1, 0, 0, 1],
    b'A' => [1, 0, 1, 0],
    b'a' => [1, 0, 1, 0],
    b'B' => [1, 0, 1, 1],
    b'b' => [1, 0, 1, 1],
    b'C' => [1, 1, 0, 0],
    b'c' => [1, 1, 0, 0],
    b'D' => [1, 1, 0, 1],
    b'd' => [1, 1, 0, 1],
    b'E' => [1, 1, 1, 0],
    b'e' => [1, 1, 1, 0],
    b'F' => [1, 1, 1, 1],
    b'f' => [1, 1, 1, 1]

};

const OCTAL_MAP_FROM_BINARY: Map<[u8; 3], u8> = phf_map! {
    [0, 0, 0] => 0,
    [0, 0, 1] => 1,
    [0, 1, 0] => 2,
    [0, 1, 1] => 3,
    [1, 0, 0] => 4,
    [1, 0, 1] => 5,
    [1, 1, 0] => 6,
    [1, 1, 1] => 7
};

const OCTAL_MAP_TO_BINARY: Map<u8, [u8; 3]> = phf_map! {
    b'0' => [0, 0, 0],
    b'1' => [0, 0, 1],
    b'2' => [0, 1, 0],
    b'3' => [0, 1, 1],
    b'4' => [1, 0, 0],
    b'5' => [1, 0, 1],
    b'6' => [1, 1, 0],
    b'7' => [1, 1, 1]
};

const BASE64_ALPHABET: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];

struct Hex {
    data: Vec<u8>,
}

impl Hex {
    fn to_binary(&self) -> Vec<u8> {
        let mut binary: Vec<u8> = Vec::with_capacity(self.data.len() * 4);
        for byte in &self.data {
            let hex = HEX_MAP.get(byte);
            match hex {
                Some(_hex) => {
                    binary.extend(_hex);
                }
                None => {
                    panic!("Unexpected error converting Hex to binary with byte: {}", byte);
                }
            }
        }
        if binary.len() % 6 == 0 {
            return binary;
        } else {
            let padding = 6 - (binary.len() % 6);
            for _ in 0..padding {
                binary.push(0);
            }
        }
        binary
    }
}

struct Octal {
    data: Vec<u8>,
}

impl Octal {
    fn from_binary(data: &[u8]) -> Self {
        let bytes: Vec<u8> = data
            .chunks(3)
            .rev()
            .map(|chunk| {
                let len = chunk.len();
                if len < 3 {
                    let mut new_chunk = vec![0; 3 - len];
                    new_chunk.extend(chunk);
                    return *OCTAL_MAP_FROM_BINARY.get(&new_chunk).unwrap();
                } else {
                    return *OCTAL_MAP_FROM_BINARY.get(chunk).unwrap();
                }
            })
            .rev()
            .collect();
        Octal { data: bytes }
    }

    fn to_base_64(&self) -> Vec<u8> {
        let mut base64: Vec<u8> = self.data
            .chunks(2)
            .map(|c| {
                println!("c: {:?}", c);
                let oct = Octal { data: c.to_vec() };
                let decimal = oct.to_decimal();
                BASE64_ALPHABET[decimal as usize]
            })
            .collect();

        if base64.len() % 4 == 0 {
            base64
        } else if (base64.len() + 1) % 4 == 0{
            base64.push(b'=');
            base64
        } else if (base64.len() + 2) % 4 == 0 {
            base64.push(b'=');
            base64.push(b'=');
            base64
        } else {
            panic!("Needed more than two padding characters to make valid base64")
        }
    }

    

    fn to_decimal(&self) -> u8 {
        self.data
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &x)| {
                let exp = 8u8.checked_pow(i as u32);
                match exp {
                    Some(exp) => exp * x,
                    None => panic!("Overflow in Octal to decimal exponentiation of 8u8 with : {}", i),
                }
            })
            .sum()
    }
}

struct Base64 {
    data: Vec<u8>,
}

impl Base64 {
    fn from_hex(hex: Hex) -> Self {
        let binary = hex.to_binary();
        let octal = Octal::from_binary(&binary);
        let base64 = octal.to_base_64();
        Base64 { data: base64 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_binary() {
        // data: 4D616E
        // target: 010011010110000101101110
        let data = vec![b'4', b'D', b'6', b'1', b'6', b'E'];
        let target = vec![
            0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0,
            0, 1, 0, 1, 1, 0, 1, 1, 1, 0,
        ];
        let hex = Hex { data };
        assert_eq!(hex.to_binary(), target);
    }

    #[test]
    fn test_octal_from_binary() {
        // data: 010011010110000101101110
        // target: 23260556
        let octal = Octal::from_binary(&[0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0,
            0, 1, 0, 1, 1, 0, 1, 1, 1, 0]);
        assert_eq!(
            octal.data,
            vec![2, 3, 2, 6, 0, 5, 5, 6]
        );
    }

    #[test]
    fn test_octal_to_base64() {
        // data 23260556
        // target TWFu
        let octal = Octal {
            data: vec![2, 3, 2, 6, 0, 5, 5, 6],
        };
        assert_eq!(octal.to_base_64(), vec![b'T', b'W', b'F', b'u']);
    }

    #[test]
    fn test_octal_to_decimal() {
        // data: 23 26 05 56
        // target: 19 22 05 46
        let octals = vec![
            Octal {
                data: vec![2, 3],
            },
            Octal {
                data: vec![2, 6],
            },
            Octal {
                data: vec![0, 5],
            },
            Octal {
                data: vec![5, 6],
            },
        ];
        let decimals: Vec<u8> = vec![19, 22, 5, 46];
        for (i, octal) in octals.iter().enumerate() {
            assert_eq!(octal.to_decimal(), decimals[i]);
        }
    }

    #[test]
    fn test_base64_from_hex() {
        // hex data: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
        // target_base64_data: SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
        
        let hex_data: Vec<u8> = vec![
            b'4', b'9', b'2', b'7', b'6', b'd', b'2', b'0', b'6', b'b', b'6', b'9', b'6', b'c',
            b'6', b'c', b'6', b'9', b'6', b'e', b'6', b'7', b'2', b'0', b'7', b'9', b'6', b'f',
            b'7', b'5', b'7', b'2', b'2', b'0', b'6', b'2', b'7', b'2', b'6', b'1', b'6', b'9',
            b'6', b'e', b'2', b'0', b'6', b'c', b'6', b'9', b'6', b'b', b'6', b'5', b'2', b'0',
            b'6', b'1', b'2', b'0', b'7', b'0', b'6', b'f', b'6', b'9', b'7', b'3', b'6', b'f',
            b'6', b'e', b'6', b'f', b'7', b'5', b'7', b'3', b'2', b'0', b'6', b'd', b'7', b'5',
            b'7', b'3', b'6', b'8', b'7', b'2', b'6', b'f', b'6', b'f', b'6', b'd',
        ];
        let target_base64_data = vec![
            b'S', b'S', b'd', b't', b'I', b'G', b't', b'p', b'b', b'G', b'x', b'p', b'b', b'm',
            b'c', b'g', b'e', b'W', b'9', b'1', b'c', b'i', b'B', b'i', b'c', b'm', b'F', b'p',
            b'b', b'i', b'B', b's', b'a', b'W', b't', b'l', b'I', b'G', b'E', b'g', b'c', b'G',
            b'9', b'p', b'c', b'2', b'9', b'u', b'b', b'3', b'V', b'z', b'I', b'G', b'1', b'1',
            b'c', b'2', b'h', b'y', b'b', b'2', b'9', b't',
        ];
        let hex = Hex { data: hex_data };
        let base64 = Base64::from_hex(hex);
        assert_eq!(base64.data, target_base64_data);

        // Hex data 2: cafebabe
        // target_base64_data: yv66vg==
        let data = vec![b'c', b'a', b'f', b'e', b'b', b'a', b'b', b'e'];
        let target_base64_data = vec![b'y', b'v', b'6', b'6', b'v', b'g', b'=', b'='];
        let hex = Hex { data };
        let base64 = Base64::from_hex(hex);
        assert_eq!(base64.data, target_base64_data);

        // Hex data 3: 4D61
        // target_base64_data: TWE=
        let data = vec![b'4', b'D', b'6', b'1'];
        let target_base64_data = vec![b'T', b'W', b'E', b'='];
        let hex = Hex { data };
        let base64 = Base64::from_hex(hex);
        assert_eq!(base64.data, target_base64_data);
    }
}
