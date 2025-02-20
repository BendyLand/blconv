extern crate image;

use crate::file;
use crate::utils;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct UnsupportedFormatError {
    format: String,
}

impl fmt::Display for UnsupportedFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Unsupported target format: {}", self.format);
    }
}

impl Error for UnsupportedFormatError {}

#[derive(PartialEq, Eq, Debug)]
pub enum ImageExt {
    PNG,
    JPEG,
    INVALID,
}

pub fn convert_image(file: String, target: ImageExt) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(&file)?;
    let new_path = file::extract_file_name(&file);
    let img_ext = match target {
        ImageExt::PNG => "png".to_string(),
        ImageExt::JPEG => "jpeg".to_string(),
        ImageExt::INVALID => "".to_string(),
    };
    let new_file = format!("{}.{}", &new_path, img_ext);
    match target {
        ImageExt::JPEG => {
            img.save_with_format(new_file.clone(), image::ImageFormat::Jpeg)?;
            println!("'{}' successfully converted to '{}'!", &file, &new_file);
        },
        ImageExt::PNG => {
            img.save_with_format(new_file.clone(), image::ImageFormat::Png)?;
            println!("'{}' successfully converted to '{}'!", &file, &new_file);
        },
        _ => return Err(Box::new(UnsupportedFormatError { format: img_ext })),
    };
    return Ok(());
}

fn find_matching_value(val: &String) -> ImageExt {
    return match val.as_str() {
       "png" => ImageExt::PNG,
       "jpeg" => ImageExt::JPEG,
       "jpg" => ImageExt::JPEG,
       &_ => ImageExt::INVALID,
    };
}

pub fn get_target_ext(args: &Vec<String>) -> Result<ImageExt, String> {
    let valid_exts = vec!["png", "jpeg", "jpg"];
    for arg in args {
        if !arg.starts_with("-") { continue; }
        let val = utils::extract_flag_value(arg);
        if valid_exts.contains(&val.as_str()) {
            return Ok(find_matching_value(&val));
        }
    }
    return Err("Target extension not provided or invalid. Use `-png`, `-jpeg`, or `-jpg`.".to_string());
}

