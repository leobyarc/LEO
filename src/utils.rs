use std::{env, fs, path::PathBuf};

use directories_next::ProjectDirs;
use uuid::Uuid;

// Generate a custom image path in the current working directory
pub fn generate_custom_image_path() -> PathBuf {
    // Obtain the current working directory
    let current_dir = env::current_dir().unwrap();
    let image_dir = current_dir.join("images");

    // Create the images directory if it doesn't already exist
    if !image_dir.exists() {
        fs::create_dir_all(&image_dir).unwrap();
    }

    // Generate a unique filename using a UUID
    let unique_file_name = format!("image-{}.png", Uuid::new_v4());
    let unique_path = image_dir.join(unique_file_name);

    unique_path
}

// Generate an image path in the application data directory
pub fn create_image_path() -> PathBuf {
    // Retrieve the application-specific directory
    let main_dirs = ProjectDirs::from("", "", "leo").unwrap();
    let image_dir = main_dirs.data_local_dir().join("images");

    // Create the images directory if it doesn't already exist
    fs::create_dir_all(&image_dir).ok().unwrap();

    // Generate a unique filename using a UUID
    let unique_file_name = format!("image-{}.png", Uuid::new_v4());
    let unique_path = image_dir.join(unique_file_name);

    unique_path
}
