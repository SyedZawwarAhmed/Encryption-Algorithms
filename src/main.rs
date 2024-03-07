mod morse_code;

use morse_code::{ decrypt, encrypt };

fn main() {
    const PLAIN_TEXT: &str = "ZAWWAR";
    // const KEY: &str = "monarchy";
    let cipher_text = encrypt(PLAIN_TEXT);
    let decipher_text = decrypt(&cipher_text);
    println!("Plain Text:- {}", PLAIN_TEXT);
    // println!("Key:- {}", KEY);
    println!("Cipher Text:- {}", cipher_text);
    println!("Decipher Text:- {}", decipher_text);
}
