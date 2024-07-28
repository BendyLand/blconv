use std::env;

pub fn get_cl_args() -> Vec<String> {
    return env::args().map(|x| x.to_string()).collect::<Vec<String>>();
}

pub fn extract_file_name(file: String) -> String {
    let idx = file.rfind(".").unwrap_or(file.len()-1);
    return file[..idx].to_string();
}

#[allow(dead_code)]
pub fn extract_file_ext(file: String) -> String {
    let idx = file.rfind(".").unwrap_or(file.len()-1);
    return file[idx+1..].to_string();
}
