use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use ureq::get;
use anyhow::Result;

// Structure representing an image with base64 encoding
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    // Base64 encoded image data
    pub base64: String,
}

impl Image {
    // Create Image from base64 string
    pub fn create_from_base64(base64: String) -> Self {
        Self { base64 }
    }

    // Create Image from file path
    pub fn create_from_file(file: String) -> Self {
        let image_data = fs::read(file).unwrap();
        let base64 = general_purpose::STANDARD.encode(&image_data);

        Self { base64 }
    }

    // Create Image from URL
    pub fn create_from_url(url: &str) -> Result<Self> {
        let response = get(url).call()?;
        let image_bytes = response.into_reader().bytes().collect::<Result<Vec<u8>, _>>()?;
        let base64 = general_purpose::STANDARD.encode(&image_bytes);

        Ok(Self { base64 })
    }

    // Save image to file system
    pub fn store(&self, path: impl Into<PathBuf>) -> Result<()> {
        let mut file = File::create(path.into()).expect("Failed to create file");
        file.write_all(&self.to_bytes()).expect("Failed save image");

        Ok(())
    }

    // Get raw bytes from base64 encoded image
    pub fn to_bytes(&self) -> Vec<u8> {
        general_purpose::STANDARD
            .decode(&self.base64)
            .expect("Failed to decode base64 string")
    }
}

// Structure for image generation request
pub struct ImageRequest {
    // Description for image generation
    pub description: String,
    // Width of the image
    pub width: u32,
    // Height of the image
    pub height: u32,
}

// Trait for image generation functionality
pub trait ImageGenerator {
    // Create image from request parameters
    fn produce_image(&self, request: ImageRequest) -> Result<Image>;
}