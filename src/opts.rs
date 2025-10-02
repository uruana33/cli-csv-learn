use clap::Parser;

use crate::CsvOpts;

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
}
