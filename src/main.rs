mod columnar_cipher;

use columnar_cipher::{ decrypt, encrypt };

fn main() {
    const PLAIN_TEXT: &str = "Zawwar";
    const KEY: &str = "HACK";
    let cipher_text = encrypt(PLAIN_TEXT, KEY);
    let decipher_text = decrypt(&cipher_text, KEY);
    println!("Plain Text:- {}", PLAIN_TEXT);
    println!("Key:- {}", KEY);
    println!("Cipher Text:- {}", cipher_text);
    println!("Decipher Text:- {}", decipher_text);
}
