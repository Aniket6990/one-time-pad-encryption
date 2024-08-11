use std::io::stdin;
mod otp_encryption;
mod utils;
fn main() {
    println!("What would you like to do:");
    println!("1. Encrypt a text");
    println!("2. Decrypt a ciper text");
    let mut choice = String::new();
    stdin().read_line(&mut choice).unwrap();
    let parsed_choice: u8 = choice.trim().parse().expect("Input type should be a integer");

    match parsed_choice {
        1 => {encrypt()}
        2 => {decrypt()}
        _ => {println!("invalid choice")}
    }
}

fn encrypt() {
    println!("Enter msg:");
    let msg = otp_encryption::get_input();
    let upper_msg = otp_encryption::to_upper_case(&msg);
    let key_index_arr = utils::generate_random_array(msg.len() - 1);
    let encrypted_bytes = otp_encryption::encrypt_msg(&upper_msg, &key_index_arr);
    let encrypted_msg = otp_encryption::print_text(encrypted_bytes);
    println!("Encrypted text: {}", encrypted_msg);
    let key = otp_encryption::print_text(key_index_arr);
    println!("key: {}",key);
    println!("msg length: {}", msg.len() - 1);
    println!("Encrypted text length: {}", encrypted_msg.len())
}

fn decrypt() {
    println!("cipher text: ");
    let ciper = otp_encryption::get_input();
    println!("key: ");
    let key = otp_encryption::get_input();
    let msg = otp_encryption::decrypt_ciper(ciper, key);
    println!("{}", msg);
}