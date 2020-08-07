use std::io::{self};

pub fn atbash_cipher() {

    let mut buffer = String::new();
    println!("1.Encrypt\n2.Decrypt\n3.Help\n0.Return to the Cipher Menu");
    io::stdin().read_line(&mut buffer).unwrap();
    let ch: i32 = buffer.trim().parse().unwrap();
    match ch {
        1 => atbash_cipher_encryptor(),
        2 => atbash_cipher_decryptor(),
        3=>println!("Help"),
        _=>println!("Error"),
    }
}

fn atbash_cipher_encryptor() {
    let mut my_str = String::new();
    println!("Enter the Sentence  : ");
    io::stdin().read_line(&mut my_str).unwrap();
    my_str.truncate(my_str.len()-1);

    let str_vec : Vec<char> = my_str.chars().collect();
    println!("Atbash Cpher Code : \n");

    for i in str_vec.iter() { 
        let integer = *i as u8;
        // earlier, this code was checking that a u8 was >= 0
        // if you change types ever (although unlikely because its a char), change this back
        if !((integer<65)||(integer>90&&integer<97)||(integer>122&&integer<=127)) {
            if integer>=65 && integer<=90 {
                let c = 90+65-integer;
                let res = c as char;
                print!("{}",res);
            }
            if integer>=97 && integer<=122 {
                let c = 122+97-integer;
                let res = c as char;
                print!("{}",res);
            }
        }

        if (integer<65)||(integer>90&&integer<97)||(integer>122&&integer<=127) {
            let res = integer as char;
            print!("{}",res);
        }
    }
    println!("\n");
}

fn atbash_cipher_decryptor() { 
    let mut my_str = String::new();
    println!("Enter the Sentence  : ");
    io::stdin().read_line(&mut my_str).unwrap();
    my_str.truncate(my_str.len()-1);

    let str_vec : Vec<char> = my_str.chars().collect();
    println!("Atbash Cpher Code : \n");

    for i in str_vec.iter() { 
        let integer = *i as u8;
        if !((integer<65)||(integer>90&&integer<97)||(integer>122&&integer<=127)) {
            if integer>=65 && integer<=90 {
                let c = 90+65-integer;
                let res = c as char;
                print!("{}",res);
            }
            if integer>=97 && integer<=122 {
                let c = 122+97-integer;
                let res = c as char;
                print!("{}",res);
            }
        }

        if (integer<65)||(integer>90&&integer<97)||(integer>122&&integer<=127) {
            let res = integer as char;
            print!("{}",res);
        }
    }
    println!("\n");
}
