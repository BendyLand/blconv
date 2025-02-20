pub fn extract_file_name(file: &String) -> String {
    let idx = file.rfind(".").unwrap_or(file.len() - 1);
    return file[..idx].to_string();
}

pub fn extract_file_ext(file: &String) -> String {
    let idx = file.rfind(".").unwrap_or(file.len() - 1);
    return file[idx + 1..].to_string();
}

#[derive(Debug)]
pub enum FileType {
    IMAGE,
    TEXT,
    AUDIO,
    PDF,
    UNKNOWN,
}

pub fn detect_file_type(file: &String) -> FileType {
    let ext = extract_file_ext(file);
    let image_exts: Vec<String> = {
        vec!["jpeg", "jpg", "png"]
            .into_iter()
            .map(|x| x.to_string())
            .collect()
    };
    let audio_exts: Vec<String> = {
        vec!["mp3", "wav", "m4a"]
            .into_iter()
            .map(|x| x.to_string())
            .collect()
    };
    let txt_exts: Vec<String> = {
        vec!["txt", "md", "json", "yaml", "yml", "csv", "html"]
            .into_iter()
            .map(|x| x.to_string())
            .collect()
    };
    let result: FileType;
    if image_exts.contains(&ext) {
        result = FileType::IMAGE;
    } 
    else if audio_exts.contains(&ext) {
        result = FileType::AUDIO;
    }
    else if txt_exts.contains(&ext) {
        result = FileType::TEXT;
    }
    else if ext == "pdf" {
        result = FileType::PDF;
    }
    else {
        result = FileType::UNKNOWN;
    }
    return result;
}
