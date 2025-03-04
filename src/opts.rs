/// cli相关模块
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name="rcli", author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    CSV(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verity_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value = ",")]
    pub delimiter: char,
}

fn verity_input_file(file_name: &str) -> Result<String, &'static str> {
    if std::path::Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("文件不存在")
    }
}
