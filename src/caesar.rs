use std::io::{self,Read};

pub fn casesar_cipher() {

    let mut buffer = String::new();
    println!("1.Encrypt\n2.Decrypt\n3.Bruteforce for all Shifts\n4.Help\n0.Return to the Cipher Menu");
    io::stdin().read_line(&mut buffer).unwrap();
    let ch: i32 = buffer.trim().parse().unwrap();
    match ch {
        1 => caesar_cipher_encryptor(),
        2 => caesar_cipher_decryptor(),
        3 => caesar_bruteforcer(),
        4=>println!("Help"),
        0 => return,
        _=>println!("Error"),
    }
}

fn caesar_cipher_encryptor() {
    let mut buffer = String::new();
    let mut message = String::new();
    let mut enc = String::new();
    let mut ch : u8;
    let key: u8;
    let mut temp : u8;

    println!("Enter the message to encrypt : ");
    io::stdin().read_line(&mut message).unwrap();
    message.truncate(message.len()-1);
    println!("Enter the key/shift : ");
    io::stdin().read_line(&mut buffer).unwrap();
    key = buffer.trim().parse().unwrap();

    let str_vec : Vec<char> = message.chars().collect();

    for i in str_vec.iter() {
        ch  = *i as u8;
        if ch>=97 && ch<=122 {
            temp = ch+key;
            if temp>122 {
                ch = temp-26;
            }
            else {
                ch = temp;
            }
            let chr = ch as char;
            enc.push(chr);
        }
        if ch>=65 && ch<=90 {
            ch = ch+key;
            if ch>90 {
                ch = ch-90+65-1
            }
            let chr = ch as char;
            enc.push(chr);
        }
    }
    println!("Encrypted Message : {}",enc);
    println!();
    return;
}

fn caesar_cipher_decryptor() {
    let mut buffer = String::new();
    let mut message = String::new();
    let mut enc = String::new();
    let mut ch : u8;
    let key: u8;

    println!("Enter the message to decrypt : ");
    io::stdin().read_line(&mut message).unwrap();
    message.truncate(message.len()-1);
    println!("Enter the key/shift : ");
    io::stdin().read_line(&mut buffer).unwrap();
    key = buffer.trim().parse().unwrap();

    let str_vec : Vec<char> = message.chars().collect();

    for i in str_vec.iter() {
        ch  = *i as u8;
        if ch>=97 && ch<=122 {
            ch = ch-key;
            if ch<97 {
                ch = ch+122-97+1;
            }
            let chr = ch as char;
            enc.push(chr);
        }

        if ch>=65 && ch<=90 {
            ch = ch-key;
            if ch<65 {
                ch = ch+90-65+1;
            }
            let chr = ch as char;
            enc.push(chr);
        }
    }
    println!("Decrypted Message : {}",enc);
    println!();
    return;
}

fn caesar_bruteforcer() {
    let mut message = String::new();
    println!("Enter the message to decrypt : ");
    io::stdin().read_line(&mut message).unwrap();
    message.truncate(message.len()-1);
    let mut enc = String::new();

    let mut str_vec : Vec<char> = message.chars().collect();
    
    for _i in 1..26 {
        for j in str_vec.iter() {
            let mut ch = *j as u8;

            if ch>=97 && ch<=122 {
                ch = ch-1;
                if ch<97 {
                    ch = ch+122-97+1;
                }
                let chr = ch as char;
                enc.push(chr);
            }
            
            if ch>=65 && ch<=90 {
                ch = ch-1;
                if ch<65 {
                    ch = ch+90-65+1;
                }
                let chr = ch as char;
                enc.push(chr);
            }
        }
        println!("{}",enc);
        message = enc.clone();
        io::empty().read_to_string(&mut enc).unwrap();
        enc = String::new();
        str_vec = message.chars().collect();
        
    }
    return;
}