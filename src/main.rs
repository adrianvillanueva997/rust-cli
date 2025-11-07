use core::fmt;
use std::{fs::File, path::Path};

use clap::Parser;
use csv::ReaderBuilder;

enum SupportedFiles {
    Csv,
    Json,
}

impl fmt::Display for SupportedFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedFiles::Csv => write!(f, "csv"),
            SupportedFiles::Json => write!(f, "json"),
        }
    }
}

fn process_csv_file(path: &Path) {
    let file_content = File::open(path).unwrap();
    let mut reader = csv::ReaderBuilder::new().delimiter(b',').has_headers(true);
}
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.file);
    if !path.exists() {
        panic!("The file {} does not exist", args.file)
    }
    if let Some(file_extension) = path.extension() {
    } else {
        panic!("Extension not supported")
    }

    println!("Hello, world!");
}
