mod playfair_cipher;

use playfair_cipher::{ decrypt, encrypt };

fn main() {
    const PLAIN_TEXT: &str = "gatlmzclrqtx";
    const KEY: &str = "monarchy";
    let cipher_text = encrypt(PLAIN_TEXT, KEY, 'z').unwrap();
    let decipher_text = decrypt(&cipher_text, KEY).unwrap();
    println!("Plain Text:- {}", PLAIN_TEXT);
    println!("Key:- {}", KEY);
    println!("Cipher Text:- {}", cipher_text);
    println!("Decipher Text:- {}", decipher_text);
}
