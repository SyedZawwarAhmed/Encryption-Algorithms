mod ceasar_cipher;

use ceasar_cipher::{encrypt, decrypt};

fn main() {
    const PLAIN_TEXT: &str = "Zawwar";
    let cipher_text = encrypt(PLAIN_TEXT, 1);
    println!("{}", PLAIN_TEXT);
    println!("{}", cipher_text);
    println!("{}", decrypt(&cipher_text, 1));
}
