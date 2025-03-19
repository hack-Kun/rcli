use std::str::FromStr;

use super::verity_input_dir;
use super::verity_input_file;
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum TextSubCommand {
    Encry(TextEncry),
    Verify(TextVerify),
    Genpass(GenPassOpts),
}

#[derive(Debug, Parser)]
pub struct TextEncry {
    // 输入 可以是文件路径或者是字符串（从stdin中获取）,需要做验证，看参数是"-"还是文件路径,如果是"-"则从stdin中获取
    /// 输入要进行签名的内容，可以是从文件中获取，也可以是直接输入的字符串,从文件中获取需要显示声明--key=file:xxxx.txt
    #[arg(long, default_value = "fixtures/black64.txt", value_parser = verity_input_file)]
    pub key: String,
    /// 要加密的数据
    #[arg(short, long, default_value = "-", value_parser = verity_input_file)]
    pub input: String,
    #[arg(long, default_value = "black3")]
    pub format: TextFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerify {
    /// 公钥或私钥的文件路径
    #[arg(long, default_value = "fixtures/black64.txt", value_parser = verity_input_file)]
    pub key: String,
    /// 要验证的签名文件的内容
    #[arg(short, long)]
    pub sign: String,
    /// 原始数据
    #[arg(short, long, default_value = "-", value_parser = verity_input_file)]
    pub input: String,
    #[arg(long, default_value = "black3")]
    pub format: TextFormat,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(long, default_value = "black3")]
    pub format: TextFormat,
    #[arg(long, default_value = "fixtures", value_parser = verity_input_dir)]
    pub output: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextFormat {
    Black3,
    Ed25519,
}

impl FromStr for TextFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "black3" => Ok(TextFormat::Black3),
            "ed25519" => Ok(TextFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextFormat> for &'static str {
    fn from(value: TextFormat) -> Self {
        match value {
            TextFormat::Black3 => "black3",
            TextFormat::Ed25519 => "ed25519",
        }
    }
}
