// this is done in order to remove ambiguation between base64.rs and crate::base64
// you can change the name of this file if you want to have "use crate::base64"
use ::base64;

use base64::{encode, decode};
use std::io::{self};

pub fn base64() {
    let mut buffer = String::new();
    println!("1.Encrypt\n2.Decrypt\n3.Help\n0.Return to the Cipher Menu");
    io::stdin().read_line(&mut buffer).unwrap();
    
    let ch: i32 = buffer.trim().parse().unwrap();
        match ch {
        0 => println!("Returning..."),
        1 => encode_wrapper(),
        2 => decode_wrapper(),
        3 => println!("Help"),
        _ => println!("Error"),
    }
}

fn encode_wrapper() {
    let mut to_encode = String::new();
    println!("Enter a string to encode to Base64: ");
    io::stdin().read_line(&mut to_encode).unwrap();

    let encoded_string = encode(to_encode);
    println!("{}", encoded_string);
}

fn decode_wrapper() {
    let mut buffer = String::new();
    println!("Enter a string to decode from Base64: ");
    io::stdin().read_line(&mut buffer).unwrap();
    // get rid of a trailing newline
    buffer.pop();

    let to_decode: &str = &*buffer;
    let bytes = decode(to_decode).unwrap();
    match String::from_utf8(bytes) {
        Ok(decoded_string) => println!("{}", decoded_string),
        Err(_err) => panic!("Failed to decode string from utf8"),
    };
}