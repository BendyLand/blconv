extern crate image;

use image::{error::UnsupportedError, GenericImageView};
use std::path::Path;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct UnsupportedFormatError {
    format: String,
}

impl fmt::Display for UnsupportedFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unsupported output format: {}", self.format)
    }
}

impl Error for UnsupportedFormatError {}



pub fn convert_image(file: String, target: String) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(&file)?;
    let idx = file.chars().position(|x| x == '.').unwrap_or(file.len()-1);
    let new_path = file[0..idx].to_string();
    let new_file = format!("{}.{}", &new_path, target);
    match target.to_lowercase().as_str() {
        "jpg" | "jpeg" => img.save_with_format(new_file, image::ImageFormat::Jpeg)?,
        "png" => img.save_with_format(new_file, image::ImageFormat::Png)?,
        _ => return Err(Box::new(UnsupportedFormatError { format: target })),
    };

    return Ok(());
}