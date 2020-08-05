use crate::Inverse;
use crate::GCD;
use std::io::{self,Read};

pub fn affine_cipher(){
    let mut buffer = String::new();
    println!("1.Encrypt\n2.Decrypt\n3.Help\n0.Return to the Cipher Menu");

    io::stdin().read_line(&mut buffer).unwrap();
    let mut ch: i32 = buffer.trim().parse().unwrap();
        match ch {
        1 => affine_cipher_encryptor(),
        2 => affine_cipher_decryptor(),
        3 => println!("Help"),
        _=>println!("Error"),

    }

}

fn affine_cipher_encryptor() {
    let mut buffer = String::new();
    let (mut i,mut j,mut k,mut gcd,mut alpha,mut beta,mut c) : (i32,i32,i32,i32,i32,i32,i32);
    let mut numstr : Vec<i32> = Vec::new();
    let mut numcipher : Vec<i32> = Vec::new();
    let mut my_str = String::new();
    let mut my_str2 = String::new();
    let mut cipher = String::new();
    let mut my_str3 = String::new();

    println!("Enter a string: ");
    io::stdin().read_line(&mut my_str);
    my_str.truncate(my_str.len()-1);
    my_str2 = my_str.clone();

    //Converting the string into Upper Case

    my_str = my_str.to_uppercase();

    println!("Enter the alpha value (Must be between 1 and 25 both inclusive) :  \n ");
    io::stdin().read_line(&mut buffer).unwrap();
    alpha = buffer.trim().parse().unwrap();
    println!("Alpha {} ",alpha);

    if alpha < 1 ||  alpha > 25 {
        println!("ABE GANDU ALPHA 1 AAR 25 ER MAJHKANE HOBE !!!!!!!!!");
        return ;
    }
    gcd = GCD::CalcGCD(alpha);

    if gcd != 1 {
        println!("Sorry Try again\n");
        return ;
    }

    println!("Enter the Beta value and they must be between 0 and 25 both inclusive : \n");
    io::empty().read_to_string(&mut buffer).unwrap();
//    assert!(buffer.is_empty());
    buffer = String::new();     //Setting the buffer agains as a String which can then be converted into an integer after taking input
    io::stdin().read_line(&mut buffer).unwrap();
    beta = buffer.trim().parse().unwrap();
    println!("Beta : {} ",beta);

    if beta < 0 || beta > 25 {
        println!("BARA GANDU TOR DARA KICHU HOBEI NA LEWRA!!!!!! \n CONDITION GULO DEKH WARA");
        return ;
    }

    //Condition over 
    //Program starts

    let str_vec : Vec<char> = my_str.chars().collect();

    for i in str_vec.iter() {
        if *i != ' ' {
            let mut integer = *i as i32;
            integer = integer-65;
            numstr.push(integer);
        }

        else{
            let mut integer = *i as i32;
            integer = integer-20;
            numstr.push(integer);
        }
    }

    //Ciphering Process
    //If numcipher is more than 25 .We need to convert and ensure that lie in between 0 and 25.(indicating Alphabets)
    //A-0,B-1,C-2,.....Y-24,Z-25

    println!("Affine Cipher text is : \n");

    let mut index : i32 = 0;
    for i in numstr.iter() {
        if *i != -20 {
            let mut temp = *i;
            numcipher.push(((alpha*temp)+beta)%26);
            let mut temp1 = numcipher[index as usize] as u8;
            temp1 = temp1+65;
            let mut c = temp1 as char;
            my_str3.push(c);
       }
        else {
            my_str3.push(' ');
        }
        index = index+1;
    }

    let str_vec2 : Vec<char> = my_str2.chars().collect();
    let str_vec3 : Vec<char> = my_str3.chars().collect();
    index = 0;
    for i in str_vec2.iter() {
        let mut temp = *i as i32;
        let mut temp1 = str_vec3[index as usize];

        if (temp>=65 && temp<=90) || (temp>=97 && temp<=122) || temp==32 {
            if temp>=97 && temp<=122 {
                print!("{}",temp1.to_lowercase());
            }
            else {
                print!("{}",temp1);
            }
        }
        index = index+1;
    }
    println!("\n");
}


fn affine_cipher_decryptor() {
    let mut buffer = String::new();
    let (mut i,mut j,mut k,mut gcd,mut alpha,mut beta,mut alphaInverse) : (i32,i32,i32,i32,i32,i32,i32);
    let mut numstr : Vec<i32> = Vec::new();
    let mut numcipher : Vec<i32> = Vec::new();
    let mut my_str = String::new();
    let mut my_str2 = String::new();
    let mut cipher = String::new();
    let mut my_str3 = String::new();

    println!("Enter the encrypted string : ");
    io::stdin().read_line(&mut my_str);
    my_str.truncate(my_str.len()-1);
    my_str2 = my_str.clone();
    //Converting the string into Upper Case

    my_str = my_str.to_uppercase();

    println!("Enter the alpha value (Must be between 1 and 25 both inclusive) :  \n ");
    io::stdin().read_line(&mut buffer).unwrap();
    alpha = buffer.trim().parse().unwrap();
    println!("Alpha {} ",alpha);

    if alpha < 1 ||  alpha > 25 {
        println!("ABE GANDU ALPHA 1 AAR 25 ER MAJHKANE HOBE !!!!!!!!!");
        return ;
    }
    gcd = GCD::CalcGCD(alpha);

    if gcd != 1 {
        println!("Sorry Try again\n");
        return ;
    }

    println!("Enter the Beta value and they must be between 0 and 25 both inclusive : \n");
    io::empty().read_to_string(&mut buffer).unwrap();
//    assert!(buffer.is_empty());
    buffer = String::new();     //Setting the buffer agains as a String which can then be converted into an integer after taking input
    io::stdin().read_line(&mut buffer).unwrap();
    beta = buffer.trim().parse().unwrap();
    println!("Beta : {} ",beta);

    if beta < 0 || beta > 25 {
        println!("BARA GANDU TOR DARA KICHU HOBEI NA LEWRA!!!!!! \n CONDITION GULO DEKH WARA");
        return ;
    }

    //For Decryption, we need to find the multiplicative inverse of alpha
    alphaInverse = Inverse::GetMultiplicativeInverse(alpha);
   //printf("MI=%d\n",alphaInverse);
     //Deciphering Process
 	//If numcipher is more than 25 .We need to convert and ensure that lie in between 0 and 25.(indicating Alphabets)
    //A-0,B-1,C-2,.....Y-24,Z-25 
    
    println!("Affine Cipher text is : \n");
    let str_vec : Vec<char> = my_str.chars().collect();
    for i in str_vec.iter() {
        if *i != ' ' {
            let mut integer = *i as i32;
            integer = integer-65;
            numstr.push(integer);
        }

        else{
            let mut integer = *i as i32;
            integer = integer-20;
            numstr.push(integer);
        }
    }
    let mut index : i32 = 0;
    for i in numstr.iter() {
        if *i != -20 {
            let mut temp = *i;
            numcipher.push(((alphaInverse*temp)+beta)%26);
            let mut temp1 = numcipher[index as usize] as u8;
            temp1 = temp1+65;
            let mut c = temp1 as char;
            my_str3.push(c);
       }
        else {
            my_str3.push(' ');
        }
        index = index+1;
    }

    let str_vec2 : Vec<char> = my_str2.chars().collect();
    let str_vec3 : Vec<char> = my_str3.chars().collect();
    index = 0;
    for i in str_vec2.iter() {
        let mut temp = *i as i32;
        let mut temp1 = str_vec3[index as usize];

        if (temp>=65 && temp<=90) || (temp>=97 && temp<=122) || temp==32 {
            if temp>=97 && temp<=122 {
                print!("{}",temp1.to_lowercase());
            }
            else {
                print!("{}",temp1);
            }
        }
        index = index+1;
    }
    println!("\n");

}