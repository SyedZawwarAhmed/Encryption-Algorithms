fn create_2d_array(i: i32, j: usize) -> Vec<Vec<String>> {
    let mut array = Vec::new();
    for _ in 0..i {
        let mut sub_array = Vec::new();
        for _ in 0..j {
            sub_array.push(String::new());
        }
        array.push(sub_array);
    }
    array
}

pub fn encrypt(plain_text: &str, key: i32) -> String {
    let mut cipher_text = String::new();
    let mut array = create_2d_array(key, plain_text.len());


    let mut j = 0;
    let mut go_down = true;
    for i in 0..plain_text.len() {
        let character = plain_text.chars().nth(i).unwrap().to_string();
        array[j as usize][i] = character;
        if go_down {
            j += 1;
        } else {
            j -= 1;
        }
        if j == 0 {
            go_down = true;
        }
        if j == (key - 1) as usize {
            go_down = false;
        }
    }

    for (_, sub_array) in array.iter().enumerate() {
        for (_, element) in sub_array.iter().enumerate() {
            cipher_text += element;
        }
    }

    return cipher_text;
}

pub fn decrypt(cipher_text: &str, key: i32) -> String {
    let mut decipher_text = String::new();
    let mut array = create_2d_array(key, cipher_text.len());

    let mut j = 0;
    let mut go_down = true;
    for i in 0..cipher_text.len() {
        array[j as usize][i] = "*".to_string();
        if go_down {
            j += 1;
        } else {
            j -= 1;
        }
        if j == 0 {
            go_down = true;
        }
        if j == (key - 1) as usize {
            go_down = false;
        }
    }

    let mut character_index  = 0;
    for i in 0..key {
        for j in 0..cipher_text.len() {
            if array[i as usize][j] == "*" {
                let character = cipher_text.chars().nth(character_index).unwrap();
                array[i as usize][j] = character.to_string();
                character_index += 1;
            }
        }
    }

    let mut j = 0;
    let mut go_down = true;
    for i in 0..cipher_text.len() {
        decipher_text += &array[j as usize][i];
        if go_down {
            j += 1;
        } else {
            j -= 1;
        }
        if j == 0 {
            go_down = true;
        }
        if j == (key - 1) as usize {
            go_down = false;
        }
    }
    return decipher_text;
}
