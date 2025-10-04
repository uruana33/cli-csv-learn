use crate::models::Base64Format;
use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::prelude::*;
use std::fs::File;
use std::io::{self, Read};

pub fn read_content(input: &str) -> Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = if input == "-" {
        println!("read from stdin");
        Box::new(io::stdin())
    } else {
        println!("read from file: {}", input);
        Box::new(File::open(input)?)
    };
    let mut buffer = Vec::new();
    println!("begin read");
    reader.read_to_end(&mut buffer)?;
    if buffer.iter().next_back().unwrap() == &b'\n' {
        buffer.pop();
    }
    println!("read content: {:?}", buffer);
    Ok(buffer)
}

pub fn encode(input: &str, format: Base64Format) -> Result<String> {
    match format {
        Base64Format::Standard => Ok(BASE64_STANDARD.encode(read_content(input)?)),
        Base64Format::UrlSafe => Ok(URL_SAFE_NO_PAD.encode(read_content(input)?)),
    }
}

pub fn decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    match format {
        Base64Format::Standard => Ok(BASE64_STANDARD.decode(read_content(input)?)?),
        Base64Format::UrlSafe => Ok(URL_SAFE_NO_PAD.decode(read_content(input)?)?),
    }
}
