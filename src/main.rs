use clap::Parser;
use rcli::{
    generate, process_csv, process_decode, process_encode, sign, Base64SubCommand, Opts,
    SubCommand, TextSubCommand,
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
            let password = generate(
                pass.length,
                pass.no_lower,
                pass.no_upper,
                pass.no_number,
                pass.no_symbol,
            )?;
            println!("{}", password);
        }
        SubCommand::Base64(b64) => match b64 {
            Base64SubCommand::Encoding(encode) => {
                let res = process_encode(&encode.input, encode.format)?;
                println!("{}", res);
            }
            Base64SubCommand::Decoding(decode) => {
                let res = process_decode(&decode.input, decode.format)?;
                println!("{}", res);
            }
        },
        SubCommand::Text(text) => match text {
            TextSubCommand::Encry(encry) => {
                sign(encry.key, encry.input, encry.format)?;
            }
            TextSubCommand::Verify(verify) => {
                rcli::verify(verify.key, verify.input, verify.format, verify.sign)?;
            }
            TextSubCommand::Genpass(genpass) => {
                rcli::generate_key(genpass.format, genpass.output)?;
            }
        },
    };
    Ok(())
}
