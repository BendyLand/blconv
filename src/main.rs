mod convert;
mod my_image;
mod utils;
mod file;

fn main() {
    let args = utils::get_cl_args();
    if args.len() > 2 {
        let file_type = file::detect_file_type(&utils::get_filename(&args));
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
        panic!("Invalid usage.");
    }
    else {
        utils::print_usage();
        panic!("Invalid usage.");
    }
}
