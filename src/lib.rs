pub const VERSION: &str = "v0.0.0";

mod shared;
pub mod encoder;
pub mod decoder;

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::{ Read, Write };

    #[test]
    fn test_encode() {
        let expected = "bGlnaHQgd29yaw==";

        let mut out_encode: Vec<u8> = Vec::new();

        let input = String::from("light work");

        if let Err(e) = encoder::encode(&mut input.as_bytes(), &mut out_encode) {
            println!("{}", e);
            std::process::exit(1);
        }
    
        let out_encode_str = match String::from_utf8(out_encode) {
            Ok(o) => o,
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        };
    
        assert_eq!(expected, out_encode_str);
    }

    #[test]
    fn test_decode() {
        let expected = "wuriyanto";

        let mut out_decode: Vec<u8> = Vec::new();

        let input = String::from("d3VyaXlhbnRv");

        if let Err(e) = decoder::decode(&mut input.as_bytes(), &mut out_decode) {
            println!("{}", e);
            std::process::exit(1);
        }
    
        let out_decode_str = match String::from_utf8(out_decode) {
            Ok(o) => o,
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        };
    
        assert_eq!(expected, out_decode_str);
    }
}