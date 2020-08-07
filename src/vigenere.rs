use std::ascii::AsciiExt;
use std::io::{self,Read};

pub fn vigenere_cipher() {
    let mut choice : i32;
    let mut buffer = String::new();
    println!("1.Encrypt\n2.Decrypt\n3.Help\n0.Exit");
    io::stdin().read_line(&mut buffer).unwrap();
    choice = buffer.trim().parse().unwrap();

    match choice {
        1=> vigenere_cipher_encryptor(),
        2=>vigenere_cipher_decryptor(),
        3=>println!("Help"),
        _=>println!("Error"),
    }
}

fn vigenere_cipher_encryptor() {
    let mut numstr : Vec<u8> = Vec::new(); //This stores the string in form of ascii
    let mut numcipher : Vec<u8> = Vec::new(); //This stores the ciphered string in form on number
    let mut numkey : Vec<u8> = Vec::new(); //This stores the key in form of number
    let mut num_enc : Vec<u8> = Vec::new(); //This stores the final ciphered string in the form of number
    let mut my_str = String::new(); //Original string
    let mut my_str2 = String::new(); //Copy of the original string to be used later on
    let mut my_str3 = String::new(); //Stores the final form of the string
    println!("Enter the string to encrypt :  ");
    io::stdin().read_line(&mut my_str);
    my_str.truncate(my_str.len()-1);

    my_str2 = my_str.clone(); //Copying the original string into the second one
    my_str = my_str.to_uppercase(); //Converting the original string to UpperCase

    let str_vec : Vec<char> = my_str.chars().collect(); //Storing the characters in the original string into a vector
    let mut str_vec3 : Vec<char> = Vec::new();
    let mut str_vec2 : Vec<char> = my_str2.chars().collect(); //Contains the characters of the cloned string
    //Storing in terms of ascii

    for i in str_vec.iter() {
         let mut ch = *i as u8;
         ch = ch-65;
         numstr.push(ch);
    }

    println!("Enter the key : ");
    let mut key = String::new();
    io::stdin().read_line(&mut key);
    key.truncate(key.len()-1);
    key = key.to_uppercase(); //Converting the key to Capital letters
    println!("The key you entered : {}",key);

    //Assigning key to the string

    let key_vec : Vec<char> = key.chars().collect();

    let str_len : usize = str_vec.len();
    let key_len : usize = key_vec.len(); 
    println!("Length of the string vector is : {}",str_len);

    let mut ii : usize = 0;
    let mut jj : usize = 0;
    let mut ch : u8;
    while ii<str_len{
        while (jj<key_len)&&(ii<str_len){
            ch  = key_vec[jj] as u8;
            ch = ch - 65;
            numkey.push(ch);
            ii = ii+1;
            jj = jj+1;
        }
        jj = 0;
    }
    //Step2
    
    let mut c : usize = 0;
    for k in numstr.iter() {
        ch = *k;
        ch = ch + numkey[c];
        numcipher.push(ch);
        c = c+1;
    }

    //Step 3

    c = 0;
    for i in numcipher.iter() {
        ch = *i;
        if ch > 25 {
            ch = ch-26;
            num_enc.push(ch);
            c = c+1;
        }
    }

    print!("Vigenere Cipher text is : ");

    for i in numcipher.iter() {
        ch = *i;
        ch = ch+65;
        let chr : char = ch as char;
        str_vec3.push(chr);
    }

   c=0;
   for i in str_vec2.iter() {
       ch = *i as u8;
       let mut chr = str_vec3[c];
       if (ch >= 65 && ch <= 90)||(ch >= 97 && ch <= 122) {
           if ch >= 97 && ch <= 122 {
               chr = AsciiExt::to_ascii_lowercase(&chr);
               print!("{}",chr);
               c = c+1;
           }
           else {
               print!("{}",chr);
               c = c+1;
           }
       }
   } 
   println!("\n");
}


// Vingenere Cipher decryptor

fn vigenere_cipher_decryptor() {
    let mut numstr : Vec<u8> = Vec::new(); //This stores the string in form of ascii
    let mut numcipher : Vec<u8> = Vec::new(); //This stores the ciphered string in form on number
    let mut numkey : Vec<u8> = Vec::new(); //This stores the key in form of number
    let mut num_enc : Vec<u8> = Vec::new(); //This stores the final ciphered string in the form of number
    let mut my_str = String::new(); //Original string
    let mut my_str2 = String::new(); //Copy of the original string to be used later on
    let mut my_str3 = String::new(); //Stores the final form of the string
    println!("Enter the string to decrypt :  ");
    io::stdin().read_line(&mut my_str);
    my_str.truncate(my_str.len()-1);

    my_str2 = my_str.clone(); //Copying the original string into the second one
    my_str = my_str.to_uppercase(); //Converting the original string to UpperCase

    let str_vec : Vec<char> = my_str.chars().collect(); //Storing the characters in the original string into a vector
    let mut str_vec3 : Vec<char> = Vec::new();
    let mut str_vec2 : Vec<char> = my_str2.chars().collect(); //Contains the characters of the cloned string
    //Storing in terms of ascii

    for i in str_vec.iter() {
         let mut ch = *i as u8;
         ch = ch-65;
         numstr.push(ch);
    }

    println!("Enter the key : ");
    let mut key = String::new();
    io::stdin().read_line(&mut key);
    key.truncate(key.len()-1);
    key = key.to_uppercase(); //Converting the key to Capital letters
    println!("The key you entered : {}",key);

    //Assigning key to the string

    let key_vec : Vec<char> = key.chars().collect();

    let str_len : usize = str_vec.len();
    let key_len : usize = key_vec.len(); 

    let mut ii : usize = 0;
    let mut jj : usize = 0;
    let mut ch : u8;
    while ii<str_len{
        while (jj<key_len)&&(ii<str_len){
            ch  = key_vec[jj] as u8;
            ch = ch - 65;
            numkey.push(ch);
            ii = ii+1;
            jj = jj+1;
        }
        jj = 0;
    }
    //Step2
    
    let mut c : usize = 0;
    for k in numstr.iter() {
        ch = *k;
        ch = ch - numkey[c];
        if ch < 0 {     //Accounting for the negative numbers
            ch = ch+26;
        }
        numcipher.push(ch);
        c = c+1;
    }

    //Step 3

    c = 0;
    for i in numcipher.iter() {
        ch = *i;
        if ch > 25 {
            ch = ch-26;
            num_enc.push(ch);
            c = c+1;
        }
    }

    print!("Decrypted Vigenere Cipher text is : ");

    for i in numcipher.iter() {
        ch = *i;
        ch = ch+65;
        let chr : char = ch as char;
        str_vec3.push(chr);
    }

   c=0;
   for i in str_vec2.iter() {
       ch = *i as u8;
       let mut chr = str_vec3[c];
       if (ch >= 65 && ch <= 90)||(ch >= 97 && ch <= 122) {
           if ch >= 97 && ch <= 122 {
               chr = AsciiExt::to_ascii_lowercase(&chr);
               print!("{}",chr);
               c = c+1;
           }
           else {
               print!("{}",chr);
               c = c+1;
           }
       }
   } 
   println!("\n");

}
