// src/main.rs

// Shades, a simple and lightweight command line steganography-based password manager in rust.
// Usage - Select a PNG file and embed content in it, secured with a password.
// This application uses AES-256-Gcm for encryption and Argon2 for key derivation.

mod cli;
mod cryptography;
mod gui;
mod steganography;
mod tests;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--cli" {
        // CLI mode
        cli::run_cli(args);
    } else {
        // GUI mode
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "Shades - Steganography Password Manager",
            options,
            Box::new(|_cc| Ok(Box::new(gui::ShadesGui::default()))),
        )
        .expect("Failed to launch the GUI");
    }
}
