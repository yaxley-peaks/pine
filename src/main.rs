use clap::Parser;
use line_parser::parse_line_ranges;
use pine::{read_file_into_lines, Args};
pub mod line_parser;

fn main() {
    let args = Args::parse();
    let _x = parse_line_ranges(args.lines);
    let y = read_file_into_lines(args.i_file);
    println!("{y:#?}");
}
