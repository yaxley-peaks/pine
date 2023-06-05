use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    pub i_file: String,
    #[arg(short)]
    pub lines: Vec<String>,
    #[arg(short)]
    pub o_file: String,
}
