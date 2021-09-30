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

    #[test]
    fn test_decode_jwt() {
        let input = String::from("eyJhbGdvcml0aG0iOiJITUFDLVNIQTI1NiIsImV4cGlyZXMiOjEyNzk3NDYwMDAsIm9hdXRoX3Rva2VuIjoiMjk1NjY2Njk1MDY0fDIuRXpwem5IRVhZWkJVZmhGQ2l4ZzYzUV9fLjM2MDAuMTI3OTc0NjAwMC0xMDAwMDA0ODMyNzI5MjN8LXJ6U1pnRVBJTktaYnJnX1VNUUNhRzlNdEY4LiIsInVzZXJfaWQiOiIxMDAwMDA0ODMyNzI5MjMifQ");
        let expected = String::from("{\"algorithm\":\"HMAC-SHA256\",\"expires\":1279746000,\"oauth_token\":\"295666695064|2.EzpznHEXYZBUfhFCixg63Q__.3600.1279746000-100000483272923|-rzSZgEPINKZbrg_UMQCaG9MtF8.\",\"user_id\":\"100000483272923\"}");
        
        let mut out_decode: Vec<u8> = Vec::new();

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