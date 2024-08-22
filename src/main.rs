// src/main.rs

// Shades, a simple and lightweight command line steganography-based password manager in rust.
// Usage - Select a PNG file and embed content in it, secured with a password.
// This application uses AES 256 for encryption and sha256 for key derivation.

mod encryption;
mod steganography;
mod tests;

use rpassword::read_password;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <embed|extract> <image_path>", args[0]);
        process::exit(1);
    }

    let action = &args[1];
    let image_path = if args.len() > 2 { &args[2] } else { "" };

    match action.as_str() {
        "embed" => {
            println!("Enter the content you want to embed:");
            let mut content = String::new();
            std::io::stdin()
                .read_line(&mut content)
                .expect("Failed to read content");

            println!("Enter password to encrypt content:");
            let password = read_password().expect("Failed to read password");

            match steganography::embed_encrypted_data(image_path, content.trim(), &password) {
                Ok(_) => println!("Data embedded in image successfully."),
                Err(e) => {
                    eprintln!("Failed to embed data in image: {}", e);
                    process::exit(1);
                }
            }
        }

        "extract" => {
            println!("Enter password to decrypt content:");
            let password = read_password().expect("Failed to read password");

            match steganography::extract_encrypted_data(image_path, &password) {
                Ok(decrypted_data) => println!("Extracted data: {}", decrypted_data),
                Err(e) => {
                    eprintln!("Failed to extract data: {}", e);
                    process::exit(1);
                }
            }
        }

        _ => {
            eprintln!("Invalid action. Use 'embed' or 'extract'.");
            process::exit(1);
        }
    }
}
