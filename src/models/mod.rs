mod csv;
pub use csv::{CsvOpts, OutputFormat};

mod gen_pw;
pub use gen_pw::GenPasswordOpts;

mod b64;
pub use b64::{Base64Format, Base64Subcommand};

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

    #[command(subcommand)]
    Base64(Base64Subcommand),
}

fn check_file_exists(path: &str) -> Result<String, &'static str> {
    if path == "-" || Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_file_exists() {
        assert_eq!(check_file_exists("abc.csv"), Ok("abc.csv".to_string()));
        assert_eq!(check_file_exists("-"), Ok("-".to_string()));
        assert_eq!(
            check_file_exists("not_exist.txt"),
            Err("File does not exist")
        );
    }
}
