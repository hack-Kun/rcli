mod opts;
mod process;

pub use opts::{Opts, OutputFormat, SubCommand};
pub use process::generate;
pub use process::process_csv;
