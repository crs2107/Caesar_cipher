use std :: io;
fn caesar_encryption(plain_text : String , key: u8) -> String {
    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (first + (c as u8 + key - first) % 26) as char
            } 
            else {
                c
            }
        })
        .collect()
}

fn caesar_decryption(cipher_text : String , key :u8) -> String {
    cipher_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (first + (c as u8 - key - first) % 26) as char
            } 
            else {
                c
            }
        })
        .collect()
}



fn main() {
    println!("Enter the plain text");
    let mut plain = String :: new();
    io :: stdin()
        .read_line(&mut plain)
        .expect("Failed to read");

    println!("enter your key : ");
    let mut k = String :: new();
    io :: stdin()
        .read_line(&mut k)
        .expect("Failed to read");
    let key:u8 =  k.trim().parse().expect("ERROR");

    let encrypted_text = caesar_encryption(plain,key);
    println!("Cipher text : {}",encrypted_text);
    
    println!("Deciphered text : {}",caesar_decryption(encrypted_text, key));

}
