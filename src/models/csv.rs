use crate::models::check_file_exists;
use anyhow::Result;
use clap::{arg, Parser};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

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

fn value_parser_output_format(value: &str) -> Result<OutputFormat, anyhow::Error> {
    value.parse().map_err(|e: anyhow::Error| e)
}
