mod convert;
mod my_image;
mod utils;
mod file;
mod video;

fn main() {
    let args = utils::get_cl_args();
    if args.len() > 2 {
        let file_type = file::detect_file_type(&utils::get_filename(&args).unwrap_or("NOT_FOUND".to_string()));
        convert::convert_file_type(&args, file_type);
    }
    else if args.len() == 2 {
        for arg in args {
            if arg.starts_with("-") {
                let val = utils::extract_flag_value(&arg);
                if val == "help".to_string() {
                    utils::print_usage();
                    return;
                }
            }
        }
        utils::print_usage();
        eprintln!("Invalid usage.");
    }
    else {
        utils::print_usage();
        eprintln!("Invalid usage.");
    }
}
