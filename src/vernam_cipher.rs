pub fn encrypt(plain_text: &str, key: &str) -> String {
    let mut cipher_text = String::new();

    for i in 0..plain_text.len() {
        let key = key.chars().nth(i % key.len()).unwrap();
        let value = plain_text.chars().nth(i).unwrap();

        let key_code = (key as u8) - 65;
        let text_code = (value as u8) - 65;
        let cipher_character = ((key_code + text_code) % 26) + 65;
        cipher_text.push(cipher_character as char);
    }
    return cipher_text;
}

pub fn decrypt(cipher_text: &str, key: &str) -> String {
    let mut decipher_text = String::new();

    for i in 0..cipher_text.len() {
        let key = key.chars().nth(i % key.len()).unwrap();
        let value = cipher_text.chars().nth(i).unwrap();

        let key_code: i32 = ((key as u8) - 65) as i32;
        let text_code: i32 = ((value as u8) - 65) as i32;
        let cipher_character = (((text_code - key_code + 26) % 26) + 65) as u8;
        decipher_text.push(cipher_character as char);
    }
    return decipher_text;
}
