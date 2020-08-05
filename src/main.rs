mod Inverse;
mod GCD;
mod affine;
mod atbash;
mod caesar;
mod vigenere;
mod morse;
mod railfence;
mod ascii;
mod do_rsa;
mod base64;

use std::io;
use std::process;

fn main() {

    let display_banner = "

 ██████╗██████╗ ██╗   ██╗██████╗ ████████╗ ██████╗         ███████╗ ██████╗ ██╗    ██╗   ██╗███████╗██████╗ 
██╔════╝██╔══██╗╚██╗ ██╔╝██╔══██╗╚══██╔══╝██╔═══██╗        ██╔════╝██╔═══██╗██║    ██║   ██║██╔════╝██╔══██╗
██║     ██████╔╝ ╚████╔╝ ██████╔╝   ██║   ██║   ██║        ███████╗██║   ██║██║    ██║   ██║█████╗  ██████╔╝
██║     ██╔══██╗  ╚██╔╝  ██╔═══╝    ██║   ██║   ██║        ╚════██║██║   ██║██║    ╚██╗ ██╔╝██╔══╝  ██╔══██╗
╚██████╗██║  ██║   ██║   ██║        ██║   ╚██████╔╝███████╗███████║╚██████╔╝███████╗╚████╔╝ ███████╗██║  ██║
 ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝        ╚═╝    ╚═════╝ ╚══════╝╚══════╝ ╚═════╝ ╚══════╝ ╚═══╝  ╚══════╝╚═╝  ╚═╝
                                                                                                            

";

    loop
    {
        println!("{}", display_banner);
        println!("Enter your choice");
        println!("\n1. Affine Cipher\n\n2. Atbash Cipher\n\n3. Caesar Cipher\n\n4. Vigenere Cipher\n\n5. Morse Code\n\n6. Railfence Cipher\n\n7. Ascii Code Conversion\n\n8. RSA (Encrpyt\\Decrypt)\n\n9. Base64 (Encode\\Decode)\n\n\n0. EXIT\n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut choice: i32 = input.trim().parse().unwrap();
        println!("{}",choice);

        match choice {
            0 => process::exit(0),
            1 => affine::affine_cipher(),
            2 => atbash::atbash_cipher(),
            3 => caesar::casesar_cipher(),
            4 => vigenere::vigenere_cipher(),
            5 => morse::morse_code(),
            6 => railfence::railfence_cipher(),
            7 => ascii::ascii_convert(),
            8 => do_rsa::control_function(),
            9 => base64::testFunc(),
            _=> println!("Error"),
        }
    }
}