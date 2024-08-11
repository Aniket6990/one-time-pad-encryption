use std::io;

pub fn to_upper_case(msg: &str) -> String {
    let mut msg_upper_case = String::new();
    for c in msg.chars() {
        let char_str = c.to_ascii_uppercase().to_string();
        msg_upper_case.push_str(&char_str)
    }
    msg_upper_case
}

pub fn get_input() -> String {
    let mut msg = String::new();
    io::stdin().read_line(&mut msg).unwrap();
    msg
}

pub fn encrypt_msg(msg: &str, key_index_arr: &Vec<u8>) -> Vec<u8> {
    let encrypted_bytes: Vec<u8> = (0..msg.len() - 1).map(|index| {
        let ch = msg.chars().nth(index).unwrap() as u8;
        (ch - 65 + key_index_arr[index])%26
    }).collect();
    encrypted_bytes
}

pub fn print_text(encrypted_bytes: Vec<u8>) -> String {
    let encrypted_msg: String = encrypted_bytes.iter().map(|&value| {
        let ascii_value = value + 65;
        ascii_value as char
    }).collect();
    encrypted_msg
}

pub fn decrypt_ciper(cipher_text: String, key: String) -> String {
    let msg_index_arr:Vec<u8> = (0..cipher_text.len() - 1).map(|index| {
        let cipher_ch = cipher_text.chars().nth(index).unwrap() as i8;
        let key_ch = key.chars().nth(index).unwrap() as i8;
        let mut result = cipher_ch - key_ch;
        if  result < 0 {
            result += 26;
        }
        result as u8
    }).collect();
    let msg = print_text(msg_index_arr);
    msg.to_ascii_lowercase()
}