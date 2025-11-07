use core::fmt;
use std::path::Path;

use clap::Parser;
use polars::{
    error::PolarsResult,
    frame::DataFrame,
    io::SerReader,
    prelude::{CsvReadOptions, JsonReader},
};

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

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
fn process_csv_file(path: &Path) -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path.to_str().unwrap().into()))?
        .finish()
}

// fn process_json_file(path: &Path) -> PolarsResult<DataFrame> {}
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
