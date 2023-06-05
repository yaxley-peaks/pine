use clap::Parser;
use line_parser::parse_line_ranges;
use pine::Args;
pub mod line_parser;
fn main() {
    let args = Args::parse();
    let x = parse_line_ranges(args.lines);
    println!("{x:?}");
}
