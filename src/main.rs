use clap::Parser;
use std::fs::File;
use std::io::{self, Read, ErrorKind};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let file_path = &args.file;

    if !(file_path.ends_with(".xct") || file_path.ends_with(".exacta")) {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "Invalid file extension: The file must have a .xct or .exacta extension.",
        ));
    }

    let mut file = File::open(file_path)?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    println!("Compiling Exacta code from file: {}\n\n{}", file_path, code);

    Ok(())
}
