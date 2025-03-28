use crate::cli::Base64Format;
use crate::tool::get_reader;
use anyhow::{Ok, Result};
use base64::prelude::*;
use std::io::Read;

// 编码base64
pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    // buffer.trim_ascii_end();

    // 解析base64编码格式
    let b64 = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buffer),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(buffer),
    };
    Ok(b64)
}

// 解码base64
pub fn process_decode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let result = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(buf)?,
    };
    let result = String::from_utf8(result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]

    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        process_decode(input, format).unwrap();
    }
}
