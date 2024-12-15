use std::fs::File;
use std::io::{self, Read};

pub struct Compiler;

impl Compiler {
    pub fn process_file(file_path: &str) -> io::Result<String> {
        if !(file_path.ends_with(".xct") || file_path.ends_with(".exacta")) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The file must have a .xct or .exacta extension.",
            ));
        }

        let mut file = File::open(file_path)?;
        let mut code = String::new();
        file.read_to_string(&mut code)?;

        Ok(code)
    }
}
