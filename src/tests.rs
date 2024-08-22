// src/tests.rs

#[cfg(test)]
mod tests {
    use image::{ImageBuffer, RgbImage};
    use std::fs;
    use std::path::Path;

    use crate::encryption::{decrypt_data, encrypt_data};

    // Test for PNG file embedding functionality
    const TEST_IMAGE_PATH: &str = "test.png";

    fn create_test_image() {
        let img: RgbImage = ImageBuffer::from_fn(100, 100, |_, _| image::Rgb([255, 255, 255]));
        img.save(TEST_IMAGE_PATH)
            .expect("Failed to create test image");
    }

    fn cleanup_test_image() {
        if Path::new(TEST_IMAGE_PATH).exists() {
            fs::remove_file(TEST_IMAGE_PATH).expect("Failed to clean up test image");
        }
    }

    #[test]
    fn png_test_embed_and_extract_data() {
        create_test_image();

        let data = b"test";
        let embedded_data = b"test";

        // Embed data
        crate::steganography::embed_data_in_image(TEST_IMAGE_PATH, data)
            .expect("Failed to embed data");

        // Extract data
        let extracted_data = crate::steganography::extract_data_from_image(TEST_IMAGE_PATH)
            .expect("Failed to extract data");

        assert_eq!(extracted_data, embedded_data);

        cleanup_test_image();
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
    fn png_test_cryptography_embedding() {
        create_test_image();

        let password = "test";
        let data = "Hello World!";

        let encrypted_data = encrypt_data(data, password).expect("Encryption failed.");

        crate::steganography::embed_encrypted_data(TEST_IMAGE_PATH, &encrypted_data, password)
            .expect("Failed to embed data.");

        let extracted_data =
            crate::steganography::extract_encrypted_data(TEST_IMAGE_PATH, password)
                .expect("Failed to extract data.");

        let decrypted_data = decrypt_data(&extracted_data, password).expect("Decryption failed.");

        assert_eq!(decrypted_data, data);

        cleanup_test_image();
    }
}
