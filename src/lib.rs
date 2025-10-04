mod models;
pub use models::{Base64Subcommand, Cli, Command, CsvOpts, GenPasswordOpts, OutputFormat};

mod process;
pub use process::{decode, encode, generate_password, process_csv};
