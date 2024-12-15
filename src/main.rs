mod error_handler;
mod compiler;

use clap::Parser;
use error_handler::ErrorHandler;
use compiler::Compiler;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let file_path = &args.file;

    match Compiler::process_file(file_path) {
        Ok(code) => {
            println!("Compiling Exacta code from file: {}\n\n{}", file_path, code);
        }
        Err(e) => {
            ErrorHandler::handle_error(&e);
        }
    }
}
