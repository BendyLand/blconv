use std::env;
use crate::file;

pub fn get_cl_args() -> Vec<String> {
    return env::args().map(|x| x.to_string()).collect::<Vec<String>>();
}

pub fn extract_flag_value(flag: &String) -> String {
    let start = flag.chars().position(|x| x != '-').unwrap_or(0);
    return flag[start..].to_string();
}

pub fn get_filename(args: &Vec<String>) -> String {
    for arg in args {
        if arg.starts_with("-") { continue; }
        if arg.contains(".") {
            if validate_filename(arg) {
                return arg.to_string();
            }
        }
    }
    print_usage();
    return "NO_MATCH".to_string();
    // panic!("No valid files provided.");
}

fn validate_filename(arg: &String) -> bool {
    let valid_exts: Vec<String> = {
        vec![
            "png",
            "jpg",
            "jpeg",
            "txt",
            "pdf",
            "mp3",
            "wav",
            "m4a",
            "html",
            "md",
            "json",
            "yaml",
            "yml",
            "csv",
        ].into_iter().map(|x| x.to_string()).collect()
    };
    if arg.contains(".") {
        let ext = file::extract_file_ext(arg);
        if valid_exts.contains(&ext) {
            return true;
        }    
    }
    return false;
}

pub fn print_usage() -> () {
    println!("
Usage: blconv <filename> -[target_ext]
Valid file extensions include: 
Image: jpeg, png
Other formats coming soon!
");
}
