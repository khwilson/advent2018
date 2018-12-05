use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
pub struct Config {
    /// The path of a file to parse
    #[structopt(parse(from_os_str))]
    pub filename: PathBuf,
    /// We are running as the second puzzle
    #[structopt(long = "second")]
    pub is_second_puzzle: bool,
}
