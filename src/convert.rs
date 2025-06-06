use crate::file;
use crate::my_image;
use crate::utils;
use crate::video;

pub fn convert_file_type(args: &Vec<String>, from: file::FileType) -> () {
    let filename = utils::get_filename(args).unwrap_or("NOT_FOUND".to_string());
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
        file::FileType::VIDEO => {
            match utils::get_video_target_ext_string(&args) {
                Ok(ext) => {
                    let base = file::extract_file_name(&filename);
                    let target = format!("{}.{}", base, &ext);
                    if let Err(e) = video::convert_video_rust(&filename, &target) {
                        eprintln!("Error: {}", e);
                    }
                }
                Err(e) => eprintln!("Error: {}", &e),
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
