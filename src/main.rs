use std::env;

mod images;

fn main() {
    println!("Hello, blconv!");
    let args = get_cl_args();
    if args.len() > 2 {
        let err = images::convert_image(args[1].clone(), args[2].clone()).unwrap();
        if err != () {
            println!("There was an error converting your image: {:?}", err);
        }
    }
}

fn get_cl_args() -> Vec<String> {
    return env::args().map(|x| x.to_string()).collect::<Vec<String>>();
}

fn get_file_ext(file: String) -> String {
    let parts = file.split(".").map(|x| x.to_string()).collect::<Vec<String>>();
    let result: String;
    if parts.len() > 1 { result = parts[parts.len()-1].to_string(); }
    else { result = "".to_string(); }
    return result;
}
