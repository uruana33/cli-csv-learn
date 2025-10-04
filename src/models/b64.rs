use crate::models::check_file_exists;
use clap::arg;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Encode string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = check_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = b64_format_value_parser, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = check_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = b64_format_value_parser, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn b64_format_value_parser(value: &str) -> Result<Base64Format, anyhow::Error> {
    match value.to_lowercase().as_str() {
        "standard" => Ok(Base64Format::Standard),
        "urlsafe" => Ok(Base64Format::UrlSafe),
        _ => anyhow::bail!("Invalid base64 format {}", value),
    }
}

// TODO: 实现Display, FromStr, From<Base64Format> for &'static str
