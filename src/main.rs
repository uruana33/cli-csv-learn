use anyhow::Result;
use clap::Parser;
use cli_csv::{process_csv, Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Csv(csv) => {
            process_csv(&csv.input, &csv.output)?;
        }
    }
    Ok(())
}
