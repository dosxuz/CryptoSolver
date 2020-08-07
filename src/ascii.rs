use std::io::{self};

pub fn ascii_convert() {
    let choice : i32;
    let mut buffer = String::new();
    println!("1.Convert to ASCII\n2.Convert from ASCII\n0.Exit");
    io::stdin().read_line(&mut buffer).unwrap();
    choice= buffer.trim().parse().unwrap();

    match choice {
        1 => ascii_convert_to(),
        2 => ascii_convert_from(),
        _ => println!("Exit"),
    }
}

fn ascii_convert_to(){
    let mut my_str = String::new();
    println!("Enter the text to convert to ascii : ");
    io::stdin().read_line(&mut my_str).unwrap();
    my_str.truncate(my_str.len()-1);
    let str_vec : Vec<char> = my_str.chars().collect();

    println!("The above text in ascii form is : ");

    for i in str_vec.iter() {
        let chr = *i as u8;
        print!("{} ",chr);
    }
    println!("\n");
}

fn ascii_convert_from() {
    let mut my_str = String::new();
    println!("Enter the space separated ascii numbers : ");
    io::stdin().read_line(&mut my_str).unwrap();
    my_str.truncate(my_str.len()-1);

    let split = my_str.split(" ");
    let str_vec : Vec<&str> = split.collect();

    for i in str_vec.iter() {
        let ch = *i;
        let num : u8;
        num = ch.parse().unwrap();
        let chr = num as char;
        print!("{}",chr);
    }
    println!("\n");
}