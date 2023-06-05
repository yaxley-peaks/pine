use clap::Parser;
use pine::Args;
pub mod line_parser;
fn main() {
    let args = Args::parse();
    println!("{args:?}");
    println!("Hello, world!");
}
