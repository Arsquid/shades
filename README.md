# Shades

**Shades** is a simple and lightweight steganography-based password manager built in rust. It allows you to embed encrypted data within PNG files, making it perfect for protecting your privacy.

### Note !

Shades is just a hobby project and is not recommended for production useage.

## Features

- **Steganography:** Embed content within PNG files using LSB (least significant bit).
- **Encryption:** Uses AES-256-GCM encryption for protecting the data, providing confidentiality and integrity.
- **Key derivation:** Uses Argon2 for deriving keys from your password, making it resistant to brute-force attacks.
- **Graphical user interface:** Choose between a GUI or a CLI mode based on your preference.

## Usage

Shades is easy to use. Select a PNG file and embed content in it, secured with a password. The content is encrypted before being embedded within the image, ensuring privacy even if the image is intercepted.
You can use it with a GUI or from the command line.

### GUI mode

Run the binary with no arguments to launch the graphical user interface.

### CLI mode

Run the binary with the '--cli' flag

#### Embed content

```
--cli embed <image_path>
```

You will be prompted to enter the content you want to protect and the password for encryption.

#### Extract content

```
--cli extract <image_path>
```

Enter the password used during embedding to decrypt and extract the content from the image.

## Roadmap

- [x] **PNG Steganography:** Embed data within PNG images.
- [x] **Encryption:** Secure encryption with AES-256-GCM and Argon2.
- [x] **GUI:** Add a graphical user interface for more ease of use.
- [ ] **Backup functionality:** Implement features to securely back up images.

