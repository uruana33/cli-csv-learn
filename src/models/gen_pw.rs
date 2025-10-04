use clap::{arg, Parser};

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
