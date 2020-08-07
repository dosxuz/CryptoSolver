use std::io::{self};
use rsa::{PublicKey,RSAPrivateKey,RSAPublicKey,PaddingScheme};
use rand::rngs::OsRng;

pub fn control_function() {
    let mut choice : i32;
    let mut buffer = String::new();
    println!("1.Encryption\n2.Decryption\n3.Help\n0.Exit");
    io::stdin().read_line(&mut buffer).unwrap();
    choice = buffer.trim().parse().unwrap();

    match choice {
        1 => encrypt(),
//        2 => decrypt(),
        3 => println!("Help"),
        _=>println!("Error"),
    }
}

fn encrypt() {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RSAPrivateKey::new(&mut rng,bits).expect("failed to generate a key");
    let public_key = RSAPublicKey::from(&private_key);
    
     //Encrypt
    let mut my_str = String::new();
    println!("Enter the string to be encrypted");
    io::stdin().read_line(&mut my_str);
    my_str.truncate(my_str.len()-1);
    let data = my_str.as_bytes();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = private_key.encrypt(&mut rng,padding,&data[..]).expect("failed to encrypt");
    for i in enc_data.iter() {
        print!("{}",i);
    }
}
