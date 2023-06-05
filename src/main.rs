use clap::Parser;
use line_parser::parse_line_ranges;
use pine::{read_file_into_lines, Args};

use crate::process::mark_lines;
pub mod line_parser;
pub mod process;

fn main() {
    let args = Args::parse();
    let _x = parse_line_ranges(args.lines);
    let _y = read_file_into_lines(args.i_file);
    println!("{:#?}", mark_lines(_x, _y));
}
