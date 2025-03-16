use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value = "16")]
    /// generate pass length
    pub length: u8,
    #[arg(long, action = ArgAction::SetTrue)]
    /// not include lower case
    pub no_upper: bool,
    #[arg(long, action = ArgAction::SetTrue)]
    /// not include lower case
    pub no_lower: bool,
    #[arg(long, action = ArgAction::SetTrue)]
    /// not include number
    pub no_number: bool,
    #[arg(long, action = ArgAction::SetTrue)]
    /// not include symbol
    pub no_symbol: bool,
}
