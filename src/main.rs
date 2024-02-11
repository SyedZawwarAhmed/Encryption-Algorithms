mod ceasar_cipher;
mod vigenere_cipher_b;

use vigenere_cipher_b::{decrypt, encrypt};

fn main() {
    const PLAIN_TEXT: &str = "SHEISLISTENING";
    const KEY: &str = "PASCAL";
    let cipher_text = encrypt(PLAIN_TEXT, KEY);
    let decipher_text = decrypt(&cipher_text, KEY);
    println!("Plain Text:- {}", PLAIN_TEXT);
    println!("Key:- {}", KEY);
    println!("Cipher Text:- {}", cipher_text);
    println!("Decipher Text:- {}", decipher_text);
}
