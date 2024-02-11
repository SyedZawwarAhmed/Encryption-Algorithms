pub fn encrypt(plain_text: &str, key: &str) -> String {
    let mut cipher_text = String::new();

    for i in 0..plain_text.len() {
        let key: u8 = key.chars().nth(i % key.len()).unwrap() as u8 - 65;
        let value: u8 = plain_text.chars().nth(i).unwrap() as u8 - 65;

        let cipher_character = ((key + value) % 26 + 65) as char;
        cipher_text.push(cipher_character);
    }
    return cipher_text;
}

pub fn decrypt(plain_text: &str, key: &str) -> String {
    let mut cipher_text = String::new();

    for i in 0..plain_text.len() {
        let key: u8 = key.chars().nth(i % key.len()).unwrap() as u8 - 65;
        let value: u8 = plain_text.chars().nth(i).unwrap() as u8 - 65;

        let cipher_character = (((value as i32 - key as i32 + 26) % 26 + 65) as u8) as char;
        cipher_text.push(cipher_character);
    }
    return cipher_text;
}
