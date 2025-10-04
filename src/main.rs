use anyhow::Result;
use clap::Parser;
use cli_csv::{decode, encode, generate_password, process_csv, Base64Subcommand, Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Csv(opts) => {
            let mut format = opts.format;
            let output = if let Some(output) = opts.output {
                // next_back() 直接从迭代器末尾取元素，更高效
                // last() 会消费整个迭代器再返回最后一个元素
                let suffix = output.split('.').next_back().unwrap();
                format = suffix.parse().unwrap();
                output
            } else {
                format!("output.{}", format)
            };
            process_csv(&opts.input, &output, format)?;
        }
        Command::GenPassword(opts) => {
            let password = generate_password(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.digits,
                opts.symbols,
            )?;
            println!("{}", password);
        }
        // 在标准输入中输入内容，按回车，在按ctrl+d结束
        Command::Base64(opts) => match opts {
            Base64Subcommand::Encode(opts) => {
                println!("encode input: {:?}", opts.input);
                println!("b64 encode: {:?}", encode(&opts.input, opts.format)?);
            }
            Base64Subcommand::Decode(opts) => {
                println!("decode input: {:?}", opts.input);
                let ret = decode(&opts.input, opts.format)?;
                println!("b64 decode: {:?}", String::from_utf8(ret)?);
            }
        },
    }
    Ok(())
}
