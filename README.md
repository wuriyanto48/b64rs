## b64rs

[![b64rs CI](https://github.com/wuriyanto48/b64rs/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/wuriyanto48/b64rs/actions/workflows/ci.yml)

Base64 Encoding implementation in Rust Programming Language

Spec: https://datatracker.ietf.org/doc/html/rfc4648#page-5

### Usage

Encoding
```rust
use b64rs::{ encoder, decoder };

fn main() {

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

    println!("{}", out_encode_str);

}
```

File Encoding
```rust
use b64rs::{ encoder, decoder };

fn main() {

    let mut out_encode: Vec<u8> = Vec::new();

    let mut input_file = match std::fs::File::open("haha.png") {
        Ok(o) => o,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = encoder::encode(&mut input_file, &mut out_encode) {
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

}
```

Decoding
```rust
use b64rs::{ encoder, decoder };

fn main() {

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

    println!("{}", out_decode_str);

}
```