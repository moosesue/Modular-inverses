use num_traits::ToPrimitive;
use std::io;
mod modular_inverse;
mod gcd_utils;
mod decrypt;
mod encrypt;
mod rsa_key_generation;

fn main() {
    //input p and q.
    println!("Enter the first prime: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let p: i32 = input.trim().parse().unwrap();

    println!("Enter the second prime: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let q: i32 = input.trim().parse().unwrap();

    //get public and private keys
    let (n,e,d) = match rsa_key_generation::generate_keys(p,q){
        Ok(keys) => keys,
        Err(err) => {
            eprintln!("Failed to get keys due to {}",err);
            return;
        }
    };

    println!("The public key is: ({},{})",n,e);
    println!("The private key is: ({},{})",n,d);

    //test RSA with an example
    println!("Please input the test letter to encrypt: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let m_str = input.trim();
    let m_char = match m_str.chars().next(){
        Some(message) => message,
        None => {
            eprintln!("Message is empty");
            return;
        }
    };
    let m_int = m_char as u32;

    if m_int < n as u32 { 
        let ciphertext = encrypt::rsa_encrypt(m_int,e as u32,n as u32);
        println!("Here is the encrypted letter: {}", ciphertext);

        let plaintext_big = decrypt::rsa_decrypt(ciphertext,d as u32,n as u32);
        println!("int {}",plaintext_big);
        let plaintext_u32 = match plaintext_big.to_u32(){
            Some(plaintext_int) if plaintext_int <= 126 && plaintext_int >= 32 => plaintext_int,
            Some(_) => {
                    eprintln!("Failed to match a valid character.");
                    return;
            },
            None => {
                eprintln!("No integer decrypted");
                return;
            }
        };
        println!("int {:?}",plaintext_u32);
        
        let plaintext_char = match char::from_u32(plaintext_u32){
            Some(chars) => chars,
            None => {
                eprintln!("No character decrypted.");
                return;
            }
        };
        println!("Here is the decryption check: {}", plaintext_char);
    }
    else{
        println!("Message too big for mod n.");
    }
}




