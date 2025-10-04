mod opts;
pub use opts::{Cli, Command, OutputFormat};

mod process;
pub use process::{generate_password, process_csv};
