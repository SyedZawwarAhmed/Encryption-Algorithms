fn get_table() -> Vec<Vec<char>> {
    let mut table: Vec<Vec<char>> = Vec::new();

    for i in 0..26 {
        let mut row: Vec<char> = Vec::new();
        for j in 0..26 {
            let ascii_value: u8 = (j + i) % 26 + 65;
            row.push(ascii_value as char);
            print!(" {}", ascii_value as char);
        }
        table.push(row);
        println!();
    }
    return table;
}
pub fn encrypt(plain_text: &str, key: &str) -> String {
    let table = get_table();

    let mut cipher_text = String::new();

    for i in 0..plain_text.len() {
        let key = key.chars().nth(i % key.len()).unwrap();
        let value = plain_text.chars().nth(i).unwrap();

        let row: usize = ((key as u8) - 65) as usize;
        let column: usize = ((value as u8) - 65) as usize;
        cipher_text.push(table[row][column]);
    }
    return cipher_text;
}

pub fn decrypt(cipher_text: &str, key: &str) -> String {
    let table = get_table();
    let mut decipher_text = String::new();

    for i in 0..cipher_text.len() {
        let key = key.chars().nth(i % key.len()).unwrap();
        let value = cipher_text.chars().nth(i).unwrap();

        let row: usize = ((key as u8) - 65) as usize;
        let mut column: usize = 0;
        for j in 0..26 {
            if table[row][j] == value {
                column = j;
            }
        }
        decipher_text.push(table[0][column]);
    }
    return decipher_text;
}
