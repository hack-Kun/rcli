use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let cli = Opts::parse();
    match cli.command {
        SubCommand::CSV(csv) => {
            process_csv(&csv.input, &csv.output)?;
        }
    };
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_process_csv() {}
}
