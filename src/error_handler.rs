use std::process;
use std::io;

pub struct ErrorHandler;

impl ErrorHandler {
    pub fn handle_error(error: &io::Error) {
        eprintln!("{:?}", error.kind());
        eprintln!("Error Message: {}", error);
        process::exit(1);
    }
}