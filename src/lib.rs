use clap::Parser;
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    i_file: String,
    #[arg(short)]
    lines: Vec<String>,
    #[arg(short)]
    o_file: String,
}
