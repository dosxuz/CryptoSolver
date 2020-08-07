use std::io::{self};

pub fn railfence_cipher() {
    let mut choice : i32;
    let mut buffer = String::new();
    println!("1.Encrypt\n2.Decrypt3.\n3.Help\n0.Exit");
    io::stdin().read_line(&mut buffer).unwrap();
    choice = buffer.trim().parse().unwrap();

    match choice {
        1 => railfence_cipher_encryptor(),
        2 => railfence_cipher_decryptor(),
        3 => println!("Help"),
        _=>println!("Error"),
    }
}


fn railfence_cipher_encryptor() {
    let mut buffer = String::new();
    let mut code:[[char;100];1000] = [['\x00';100];1000];
    let mut rails : usize;
    let mut my_str = String::new();
    println!("Enter a Secret message : ");
    io::stdin().read_line(&mut my_str);
    my_str.truncate(my_str.len());
    let str_vec : Vec<char> = my_str.chars().collect();

    let mut length = my_str.len();
    println!("Enter the number of rails : ");
    io::stdin().read_line(&mut buffer).unwrap();
    rails = buffer.trim().parse().unwrap();

    let mut i : usize = 0;
    let mut j : usize = 0;
    let mut count = 0;

    while j<length {
        if count%2 == 0 {
            i = 0;
            while i<rails{
                code[i][j] = str_vec[j];
                j=j+1;
                i = i+1;
            }
        }
        else {
            i=rails-2;
            while i>0 {
                code[i][j] = str_vec[j];
                j = j+1;
                i = i-1;
            } 
        }
        count = count+1;
    }

    for k in 0..rails+1 {
        for l in 0..length+1 {
            if code[k as usize][l as usize] != '\x00' && code[k as usize][l as usize] != '\n' {
                print!("{}",code[k][l]);
            }
        }
    }
    println!("\n");
}

fn railfence_cipher_decryptor() {
    let mut buffer = String::new();
    let mut code:[[char;100];1000] = [['\x00';100];1000];
    let mut rails : usize;
    let mut my_str = String::new();
    println!("Enter a Secret message : ");
    io::stdin().read_line(&mut my_str);
    my_str.truncate(my_str.len()-1);
    let str_vec : Vec<char> = my_str.chars().collect();

    let mut length = str_vec.len();
    println!("Length of the string is : {}",length);
    println!("Enter the number of rails : ");
    io::stdin().read_line(&mut buffer).unwrap();
    rails = buffer.trim().parse().unwrap();

    let mut k : usize = 0;
    let mut count : usize = 0;

    for r in 0..length {
        code[count][r as usize] = '\x7f';
        if k == 0 {
            if count == (rails-1) {
                k = 1;
                count = count-1;
            }
            else {
                count = count+1;
            }
        }
        else {
            if count == 0 {
                k = 0;
                count = count+1;
            }
            else {
                count = count-1;
            }
        }
    }

    count = 0;
    k = 0;
    for i in 0..rails {
            for j in 0..length {
                if code[i as usize][j as usize] != '\x00' {
                    code[i as usize][j as usize] = str_vec[k];
                    k = k+1;
                }
            }
    }

    count = 0;
    k = 0;

    for r in 0..length {
        if code[count][r as usize] != '\x00' {
            print!("{}",code[count][r]);
        }
        if k == 0 {
            if count == (rails-1) {
                k = 1;
                count = count-1;
            }
            else {
                count = count+1;
            }
        }
        else {
            if count == 0 {
                k = 0;
                count = count+1;
            }
            else {
                count = count-1;
            }
        }
    }

    println!("\n");
}