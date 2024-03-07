use std::collections::HashMap;

pub fn encrypt(plain_text: &str) -> String {
    let morse_code: HashMap<char, &str> = [
        ('A', ".-"), ('B', "-..."), ('C', "-.-."), ('D', "-.."), ('E', "."), ('F', "..-."),
        ('G', "--."), ('H', "...."), ('I', ".."), ('J', ".---"), ('K', "-.-"), ('L', ".-.."),
        ('M', "--"), ('N', "-."), ('O', "---"), ('P', ".--."), ('Q', "--.-"), ('R', ".-."),
        ('S', "..."), ('T', "-"), ('U', "..-"), ('V', "...-"), ('W', ".--"), ('X', "-..-"),
        ('Y', "-.--"), ('Z', "--.."), ('1', ".----"), ('2', "..---"), ('3', "...--"),
        ('4', "....-"), ('5', "....."), ('6', "-...."), ('7', "--..."), ('8', "---.."),
        ('9', "----."), ('0', "-----"), (' ', "/")
    ].iter().cloned().collect();

    let mut encrypted = String::new();
    for c in plain_text.chars() {
        if let Some(code) = morse_code.get(&c.to_ascii_uppercase()) {
            encrypted.push_str(code);
            encrypted.push(' ');
        }
    }
    encrypted.trim().to_string()
}

pub fn decrypt(cipher_text: &str) -> String {
    let morse_code: HashMap<&str, char> = [
        (".-", 'A'), ("-...", 'B'), ("-.-.", 'C'), ("-..", 'D'), (".", 'E'), ("..-.", 'F'),
        ("--.", 'G'), ("....", 'H'), ("..", 'I'), (".---", 'J'), ("-.-", 'K'), (".-..", 'L'),
        ("--", 'M'), ("-.", 'N'), ("---", 'O'), (".--.", 'P'), ("--.-", 'Q'), (".-.", 'R'),
        ("...", 'S'), ("-", 'T'), ("..-", 'U'), ("...-", 'V'), (".--", 'W'), ("-..-", 'X'),
        ("-.--", 'Y'), ("--..", 'Z'), (".----", '1'), ("..---", '2'), ("...--", '3'),
        ("....-", '4'), (".....", '5'), ("-....", '6'), ("--...", '7'), ("---..", '8'),
        ("----.", '9'), ("-----", '0'), ("/", ' ')
    ].iter().cloned().collect();

    let mut decrypted = String::new();
    for code in cipher_text.split_whitespace() {
        if let Some(&character) = morse_code.get(code) {
            decrypted.push(character);
        }
    }
    decrypted
}


