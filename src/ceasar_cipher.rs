fn get_staring_index(ascii_value: u8) -> i32 {
    if ascii_value >= 97 {
        97
    } else {
        65
    }
}

pub fn encrypt(plain_text: &str, key: i32) -> String {
    let mut cipher_text = String::new();

    for character in plain_text.chars() {
        let mut ascii_value: u8 = character as u8;
        let starting_index: i32 = get_staring_index(ascii_value);
        ascii_value -= starting_index as u8;
        ascii_value = ((ascii_value as i32 + key) % 26 + starting_index) as u8;
        let new_character: char = ascii_value as char;
        cipher_text.push(new_character);
    }

    return cipher_text;
}

pub fn decrypt(plain_text: &str, key: i32) -> String {
    encrypt(plain_text, 26 - key)
}
