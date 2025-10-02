mod opts;
pub use opts::{Cli, Command, OutputFormat};

mod process;
pub use process::process_csv;
