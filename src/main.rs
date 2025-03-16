use clap::Parser;
use rcli::{
    generate, process_csv, process_decode, process_encode, Base64SubCommand, Opts, SubCommand,
};

fn main() -> anyhow::Result<()> {
    let cli = Opts::parse();
    match cli.command {
        // 匹配csv命令执行文件解析
        SubCommand::CSV(csv) => {
            // 处理用户给定的output 参数，如果用户没有指定输出文件名，那么默认为output，如果输入参数中包含后缀，将其丢弃
            let output = if csv.output.contains(".") {
                csv.output.split(".").collect::<Vec<&str>>()[0].to_string()
            } else {
                csv.output
            };
            // 添加format参数使得用户可以解析更多种文件格式
            process_csv(&csv.input, output, csv.format)?;
        }
        SubCommand::GenPass(pass) => {
            generate(
                pass.length,
                pass.no_lower,
                pass.no_upper,
                pass.no_number,
                pass.no_symbol,
            )?;
        }
        SubCommand::Base64(b64) => match b64 {
            Base64SubCommand::Encoding(encode) => {
                process_encode(&encode.input, encode.format)?;
            }
            Base64SubCommand::Decoding(decode) => {
                process_decode(&decode.input, decode.format)?;
            }
        },
    };
    Ok(())
}
