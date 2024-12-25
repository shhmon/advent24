use std::fs;
use std::io;

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => return Err(e),
    };

    return Ok(content);
}
