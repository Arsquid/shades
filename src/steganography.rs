// src/steganography.rs

use crate::encryption::{decrypt_data, encrypt_data};
use image::{GenericImage, GenericImageView, Rgba};
use std::error::Error;

const END_MARKER: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

pub fn embed_encrypted_data(
    image_path: &str,
    data: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    let encrypted_data = encrypt_data(data, password)?;
    embed_data_in_image(image_path, encrypted_data.as_bytes())
}

pub fn extract_encrypted_data(image_path: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let extracted_data = extract_data_from_image(image_path)?;
    if extracted_data.is_empty() {
        return Err("No embedded data found".into());
    }
    decrypt_data(&String::from_utf8_lossy(&extracted_data), password)
}

pub fn embed_data_in_image(image_path: &str, data: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut img = image::open(image_path)?;

    let mut data_bits: Vec<u8> = data
        .iter()
        .flat_map(|&byte| (0..8).map(move |i| (byte >> (7 - i)) & 1))
        .collect();

    data_bits.extend(
        END_MARKER
            .iter()
            .flat_map(|&byte| (0..8).map(move |i| (byte >> (7 - i)) & 1)),
    );

    let (width, height) = img.dimensions();
    let mut bit_idx = 0;

    'outer: for y in 0..height {
        for x in 0..width {
            let mut pixel = img.get_pixel(x, y).0;

            for i in 0..3 {
                if bit_idx < data_bits.len() {
                    pixel[i] = (pixel[i] & 0xFE) | data_bits[bit_idx];
                    bit_idx += 1;
                } else {
                    img.put_pixel(x, y, Rgba(pixel));
                    break 'outer;
                }
            }
            img.put_pixel(x, y, Rgba(pixel));
        }
    }

    img.save(image_path)?;

    Ok(())
}

pub fn extract_data_from_image(image_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::open(image_path)?;
    let (width, height) = img.dimensions();

    let mut data_bits = Vec::new();

    'outer: for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0;

            for i in 0..3 {
                data_bits.push(pixel[i] & 1);
                if data_bits.len() >= 64
                    && data_bits[data_bits.len() - 64..]
                        == END_MARKER
                            .iter()
                            .flat_map(|&byte| (0..8).map(move |i| (byte >> (7 - i)) & 1))
                            .collect::<Vec<u8>>()[..]
                {
                    break 'outer;
                }
            }
        }
    }

    if data_bits.len() < 64
        || !data_bits.ends_with(
            &END_MARKER
                .iter()
                .flat_map(|&byte| (0..8).map(move |i| (byte >> (7 - i)) & 1))
                .collect::<Vec<u8>>(),
        )
    {
        return Err("No embedded data found".into());
    }

    data_bits.truncate(data_bits.len() - 64);

    let mut data = Vec::new();
    let mut byte = 0;
    let mut bits_collected = 0;

    for bit in data_bits {
        byte = (byte << 1) | bit;
        bits_collected += 1;
        if bits_collected == 8 {
            data.push(byte);
            byte = 0;
            bits_collected = 0;
        }
    }

    if bits_collected > 0 {
        data.push(byte << (8 - bits_collected));
    }

    Ok(data)
}
