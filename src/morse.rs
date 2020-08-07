use std::io::{self};

pub fn morse_code() {
    let choice : i32;
    let mut buffer = String::new();
    println!("1.Encrypt\n2.Decrypt\n3.Help0.Exit");
    io::stdin().read_line(&mut buffer).unwrap();
    choice = buffer.trim().parse().unwrap();

    match choice {
        1 => morse_code_encryptor(),
        2 => morse_code_decryptor(),
        3 => println!("Help"),
        _=>println!("Error"),
    }
}

fn morse_code_encryptor() {
    let alphamorse = vec![".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];

     let numorse = vec!["-----",".----","..---","...--","....-",".....","-....","--...","---..","----."];
     let mut my_str = String::new();
     let mut str_vec : Vec<char>;
     println!("Enter a sentence : ");
     io::stdin().read_line(&mut my_str).unwrap();
     my_str.truncate(my_str.len()-1);
     str_vec = my_str.chars().collect();

     for i in str_vec.iter() {
         let mut chr = *i as char;
         if chr!=' '&& !chr.is_digit(10){
            chr = chr.to_uppercase().collect::<Vec<_>>()[0];
            let mut ch = chr as usize;
            ch = ch-65;
            print!("{} ",alphamorse[ch]);
         }
         else if chr == ' ' {
             print!(" ");
         }
         else {
             let mut ch = chr as usize;
             ch = ch-48;
             print!("{} ",numorse[ch]);
         } 
     }
     println!("\n");
}

fn morse_code_decryptor() {
    let alphamorse = vec![".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
    let numorse = vec!["-----",".----","..---","...--","....-",".....","-....","--...","---..","----."];
    let mut morsecode = String::new();
    println!("Enter the morse code : ");
    io::stdin().read_line(&mut morsecode).unwrap();
    morsecode.truncate(morsecode.len()-1);

    //For splitting into letters of morse code

    let morse : Vec<_> = morsecode.split(' ').collect();
    let mut c : u8;

    for i in morse.iter() {
        let word = *i;
        let res = alphamorse.iter().any(|&v| v == word);
        if res {
            let mut j = alphamorse.iter().position(|&v| v == word).unwrap();
            j = j+65;
            c = j as u8;
            let chr = c as char;
            print!("{}",chr);
        }
        else {
            let j = numorse.iter().position(|&v| v == word).unwrap();
            print!("{}",j);
        }
    }
    print!("\n");
}
