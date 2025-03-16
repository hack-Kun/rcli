use std::str::FromStr;

use super::verity_input_file;
use anyhow::{Ok, Result};
use clap::{arg, Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Base64SubCommand {
    #[command(name = "encode")]
    Encoding(Base64Encode),
    #[command(name = "decode")]
    Decoding(Base64Decode),
}

#[derive(Debug, Parser)]
pub struct Base64Encode {
    #[arg(short, long, default_value = "-", value_parser = verity_input_file)]
    pub input: String,
    #[arg(long, default_value = "urlsafe")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64Decode {
    #[arg(short, long, default_value = "-", value_parser = verity_input_file)]
    pub input: String,
    #[arg(long, default_value = "urlsafe", value_parser = parser_format)]
    pub format: Base64Format,
}

// 做两种base64格式的转换
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}

impl From<Base64Format> for &str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

fn parser_format(s: &str) -> Result<Base64Format, anyhow::Error> {
    s.parse()
}
