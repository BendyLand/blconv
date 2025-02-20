use crate::file;
use crate::my_image;
use crate::utils;

pub fn convert_file_type(args: &Vec<String>, from: file::FileType) -> () {
    let filename = utils::get_filename(args);
    match from {
        file::FileType::IMAGE => {
            match my_image::get_target_ext(&args) {
                Ok(target_ext) => {
                    match my_image::convert_image(filename, target_ext) {
                        Ok(_) => {},
                        Err(e) => println!("Error: {}", e),
                    };
                },
                Err(e) => println!("{}", e),
            }
        },
        file::FileType::AUDIO => {
            println!("Audio conversion not yet implemented.");
        },
        file::FileType::TEXT => {
            println!("Text conversion not yet implemented.");
        },
        file::FileType::PDF => {
            println!("PDF conversion not yet implemented.");
        },
        file::FileType::UNKNOWN => {
            println!("Unknown file type detected.");
        },
    }
}
