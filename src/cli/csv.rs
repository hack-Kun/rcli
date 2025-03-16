use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = super::verity_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output")]
    /// 设置output参数，可以指定输出文件名，格式为json, 如果不指定输出文件名也不指定输出格式，那么将自动生成为output.json
    pub output: String,
    #[arg(short, long, default_value = ",")]
    pub delimiter: char,
    #[arg(long, default_value = "json", value_parser = parser_format)]
    /// 设置输出格式，目前支持json和yaml,如果不指定输出文件名也不指定输出格式，那么将自动生成为output.json
    pub format: OutputFormat,
}

// 类型支持
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    JSON,
    YAML,
}

// outputformat trait============================================================
// 让outputformat可以实现to_str 和 fromstr方便后续使用
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::JSON),
            "yaml" => Ok(OutputFormat::YAML),
            _ => Err(anyhow::anyhow!("不支持的输出格式")),
        }
    }
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::JSON => "json",
            OutputFormat::YAML => "yaml",
        }
    }
}

// 验证用户输入格式
fn parser_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
