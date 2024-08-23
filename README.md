# Shades

**Shades** is a simple and lightweight command-line steganography-based password manager built in rust. It allows you to embed encrypted data within PNG files, making it perfect for protecting your privacy.

### Note

! Shades is just a hobby project and is not recommended for production useage !

## Features

- **Steganography:** Embed content within PNG files using LSB (least significant bit).
- **Encryption:** Uses AES-256-GCM encryption for protecting the data, providing confidentiality and integrity.
- **Key derivation:** Uses Argon2 for deriving keys from your password, making it resistant to brute-force attacks.

## Usage

Shades is easy to use. Select a PNG file and embed content in it, secured with a password. The content is encrypted before being embedded within the image, ensuring privacy even if the image is intercepted.

### Embed content

```
cargo run embed <image_path>
```

You will be prompted to enter the content you want to protect and the password for encryption.

### Extract content

```
cargo run extract <image_path>
```

Enter the password used during embedding to decrypt and extract the content from the image.

## Roadmap

- [x] **PNG Steganography:** Embed data within PNG images.
- [x] **Encryption:** Secure encryption with AES-256-GCM and Argon2.
- [ ] **GUI:** Add a graphical user interface for easier usage.
- [ ] **Backup functionality:** Implement features to securely back up images.

