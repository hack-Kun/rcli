use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::OutputFormat;

/// csv模块

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, mut output: String, format: OutputFormat) -> anyhow::Result<()> {
    // 读取csv文件
    let mut reader = csv::Reader::from_path(input)?;
    // 创建缓存
    let mut player = Vec::with_capacity(128);
    // reader.records 读取出来的文件格式不包含头部，需要手动添加
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // 将读取出来的数据以(header,record)格式添加到集合
        let json = headers.iter().zip(record.iter()).collect::<Value>();
        player.push(json);
    }
    // 将集合转换为对应格式的字符串
    let res = match format {
        OutputFormat::JSON => {
            output = format!("{}.json", output);
            serde_json::to_string_pretty(&player)?
        }
        OutputFormat::YAML => {
            output = format!("{}.yaml", output);
            serde_yaml::to_string(&player)?
        }
    };
    // 将其写入到文件中
    fs::write(output, res)?;
    Ok(())
}
