mod base64;
mod csv;
mod genpass;

pub use base64::{Base64Format, Base64SubCommand};
use clap::{Parser, Subcommand};
pub use csv::{CsvOpts, OutputFormat};
pub use genpass::GenPassOpts;

#[derive(Debug, Parser)]
#[command(name="rcli", author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "解析csv文件")]
    CSV(CsvOpts),
    #[command(
        name = "genpass",
        about = "generate random pass, default length is 16, include upper, lower, number, symbol"
    )]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

// 验证输入文件是否存在
fn verity_input_file(file_name: &str) -> Result<String, &'static str> {
    if file_name.eq("-") || std::path::Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("文件不存在")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verity_input_file() {
        let file_name = "-";
        assert_eq!(verity_input_file(file_name).unwrap(), "-");
        let file_name = "assets/juventus.csv";
        assert_eq!(verity_input_file(file_name).unwrap(), "assets/juventus.csv");
        let file_name = "abc.json";
        assert_eq!(verity_input_file(file_name), Err("文件不存在"));
    }
}
