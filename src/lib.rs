mod cli;
mod process;
pub use cli::{Base64SubCommand, Opts, OutputFormat, SubCommand};
pub use process::{generate, process_csv, process_decode, process_encode};
