use std::fs;

pub fn get_content_from_path(path: &str) -> String {
    let content = match fs::read_to_string(path) {
        Ok(x) => x,
        Err(_) => panic!("failed to read path")
    };
    content
}