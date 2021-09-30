use b64rs::{ encoder, decoder };

fn main() {

    let mut out_encode: Vec<u8> = Vec::new();
    let mut out_decode: Vec<u8> = Vec::new();

    let input = String::from("light work");

    let mut input_file = match std::fs::File::open("haha.png") {
        Ok(o) => o,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

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

    println!("{}", out_encode_str);

    // -----------------------

    if let Err(e) = decoder::decode(&mut out_encode_str.as_bytes(), &mut out_decode) {
        println!("{}", e);
        std::process::exit(1);
    }

    let out_encode_str = match String::from_utf8(out_decode) {
        Ok(o) => o,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    println!("{}", out_encode_str);

}