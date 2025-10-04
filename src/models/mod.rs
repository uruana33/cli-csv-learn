mod csv;
pub use csv::{CsvOpts, OutputFormat};

mod gen_pw;
pub use gen_pw::GenPasswordOpts;

use anyhow::Result;
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    pub command: Command, // Command必须是enum类型
}

#[derive(Parser, Debug)]
pub enum Command {
    #[command(name = "csv", about = "CSV command")]
    Csv(CsvOpts),
    #[command(name = "gen-pw", about = "Generate password command")]
    GenPassword(GenPasswordOpts),
}

fn check_file_exists(path: &str) -> Result<String, &'static str> {
    if Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err("File {} does not exist")
    }
}
