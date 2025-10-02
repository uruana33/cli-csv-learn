use clap::{Parser};
use std::path::{Path};
use anyhow::Result;

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = check_file_exists)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value = ",")]
    pub delimiter: String,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn check_file_exists(path: &str) -> Result<String, String> {
    if Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(format!("File {} does not exist", path))
    }
}