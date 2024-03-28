use std::collections::HashMap;

pub fn encrypt(plain_text: &str, key: &str) -> String {
    let len_key = key.len();
    let len_plain = plain_text.len();
    let row = (len_plain + len_key - 1) / len_key;

    let mut matrix = vec![vec![' '; len_key]; row];

    for (i, ch) in plain_text.chars().enumerate() {
        let r = i / len_key;
        let c = i % len_key;
        matrix[r][c] = ch;
    }

    let mut key_indices: Vec<_> = key.chars().enumerate().collect();
    key_indices.sort_unstable_by_key(|&(_, ch)| ch);

    let mut cipher_text = String::new();
    for &(index, _) in &key_indices {
        for row in &matrix {
            cipher_text.push(row[index]);
        }
    }

    cipher_text
}

pub fn decrypt(encrypted_text: &str, key: &str) -> String {
    let len_key = key.len();
    let len_cipher = encrypted_text.len();
    let rows = (len_cipher + len_key - 1) / len_key;

    let mut key_indices: Vec<_> = key.chars().enumerate().collect();
    key_indices.sort_unstable_by_key(|&(_, ch)| ch);

    let mut col_lengths = vec![rows; len_key];
    for i in 0..(len_cipher % len_key) {
        col_lengths[i] -= 1;
    }

    let mut col_order: HashMap<char, usize> = HashMap::new();
    for (_, &(index, ch)) in key_indices.iter().enumerate() {
        col_order.insert(ch, index);
    }

    let mut matrix = vec![vec![' '; len_key]; rows];
    let mut idx = 0;
    for &(_, col_ch) in &key_indices {
        if let Some(&col) = col_order.get(&col_ch) {
            for r in 0..col_lengths[col] {
                if idx < len_cipher {
                    matrix[r][col] = encrypted_text.chars().nth(idx).unwrap();
                    idx += 1;
                }
            }
        }
    }

    let mut p_text = String::new();
    for r in 0..rows {
        for c in 0..len_key {
            p_text.push(matrix[r][c]);
        }
    }

    p_text.trim_end().to_string()
}
