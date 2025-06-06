use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    pub fn convert_video(input_path: *const c_char, output_path: *const c_char) -> i32;
}

pub fn convert_video_rust(input: &str, output: &str) -> Result<(), String> {
    let c_input = CString::new(input).map_err(|_| "Invalid input path".to_string())?;
    let c_output = CString::new(output).map_err(|_| "Invalid output path".to_string())?;
    let result = unsafe {
        convert_video(c_input.as_ptr(), c_output.as_ptr())
    };
    if result == 0 {
        println!("Successfully converted '{}' → '{}'", input, output);
        Ok(())
    }
    else {
        Err(format!("Conversion failed with code {}", result))
    }
}

