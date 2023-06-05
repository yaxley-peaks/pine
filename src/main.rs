use clap::Parser;
use line_parser::parse_line_ranges;
use pine::{read_file_into_lines, Args};

use crate::process::{build_final_result, mark_lines};
pub mod line_parser;
pub mod process;

fn main() {
    let args = Args::parse();
    let _x = parse_line_ranges(args.lines);
    let _y = read_file_into_lines(args.i_file);
    let _z = read_file_into_lines(args.o_file);
    let step1 = mark_lines(_x, _y);
    let step2 = build_final_result(step1, _z);
    let step2 = step2.join("\n");
    println!("{step2}");
}
