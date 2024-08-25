// src/tests.rs

#[cfg(test)]
mod tests {
    use image::{ImageBuffer, RgbImage};
    use std::fs;
    use std::path::Path;

    use crate::cryptography::{decrypt_data, encrypt_data};
    use crate::steganography::{
        embed_data_in_image, embed_encrypted_data, extract_data_from_image, extract_encrypted_data,
    };

    // Test for PNG file embedding functionality
    #[test]
    fn png_test_embed_and_extract_data() {
        let test_image_path: &str = "test01.png";

        let img: RgbImage = ImageBuffer::from_fn(100, 100, |_, _| image::Rgb([255, 255, 255]));
        img.save(test_image_path)
            .expect("Failed to create test image.");

        let data = b"test";
        let embedded_data = b"test";

        // Embed data
        embed_data_in_image(test_image_path, data).expect("Failed to embed data");

        // Extract data
        let extracted_data =
            extract_data_from_image(test_image_path).expect("Failed to extract data");

        assert_eq!(extracted_data, embedded_data);

        if Path::new(test_image_path).exists() {
            fs::remove_file(test_image_path).expect("Failed to clean up test image.");
        }
    }

    // Test for encryption / decryption functionality
    #[test]
    fn test_encryption_decryption() {
        let password = "test";
        let data = "Hello World!";

        // Encrypt the data
        let encrypted_data = encrypt_data(data, password).expect("Encryption failed.");

        // Decrypt the data
        let decrypted_data = decrypt_data(&encrypted_data, password).expect("Decryption failed.");

        assert_eq!(decrypted_data, data);
    }

    // Test for combined encryption and embedding / extraction and decryption
    #[test]
    fn png_test_cryptography_embed_extract() {
        let test_image_path: &str = "test02.png";

        let img: RgbImage = ImageBuffer::from_fn(100, 100, |_, _| image::Rgb([255, 255, 255]));
        img.save(test_image_path)
            .expect("Failed to create test image.");

        let password = "test";
        let data = "Hello World!";

        let encrypted_data = encrypt_data(data, password).expect("Encryption failed.");

        embed_encrypted_data(test_image_path, &encrypted_data, password)
            .expect("Failed to embed data.");

        let extracted_data =
            extract_encrypted_data(test_image_path, password).expect("Failed to extract data.");

        let decrypted_data = decrypt_data(&extracted_data, password).expect("Decryption failed.");

        assert_eq!(decrypted_data, data);

        if Path::new(test_image_path).exists() {
            fs::remove_file(test_image_path).expect("Failed to clean up test image.");
        }
    }
}
