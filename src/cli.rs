// src/cli.rs

use crate::steganography;
use rpassword::read_password;
use std::process;

pub fn run_cli(args: Vec<String>) {
    if args.len() < 3 {
        println!("Welcome to shades.");
        eprintln!(
            "How to use the cli-version: {} --cli <embed|extract> <image_path>",
            args[0]
        );
        process::exit(1);
    }

    let action = &args[2];
    let image_path = if args.len() > 3 { &args[3] } else { "" };

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
