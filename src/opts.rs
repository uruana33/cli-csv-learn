use anyhow::Result;
use clap::Parser;
use std::{
    fmt::{self, Display},
    path::Path,
    str::FromStr,
};

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

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = check_file_exists)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, default_value = ",")]
    pub delimiter: String,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(long, value_parser = value_parser_output_format, default_value = "json")]
    pub format: OutputFormat,
}

#[derive(Parser, Debug, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Parser, Debug, Clone)]
pub struct GenPasswordOpts {
    #[arg(long)]
    pub length: u32,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub digits: bool,

    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}

fn check_file_exists(path: &str) -> Result<String, &'static str> {
    if Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err("File {} does not exist")
    }
}

fn value_parser_output_format(value: &str) -> Result<OutputFormat, anyhow::Error> {
    value.parse().map_err(|e: anyhow::Error| e)
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => anyhow::bail!("Invalid output format {}", s),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Toml => write!(f, "toml"),
        }
    }
}
