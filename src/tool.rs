use anyhow::Result;
use std::fs::File;
use std::io::{stdin, Read};

// 从输入参数中获取文件
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    // 如果参数是-，从输入流中获取内容，否则直接从文件中获取内容
    let read: Box<dyn Read> = if input == "-" {
        Box::new(stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(read)
}
