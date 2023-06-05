use std::process::exit;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum Kind {
    Num(u32),
    Range((u32, u32)),
}

fn parse_value(val: &str) -> Kind {
    let parse_try = val.parse::<u32>();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<init>\d+)-(?P<fin>\d+)").unwrap();
    }
    // Move to regex based parsing
    match parse_try {
        Ok(v) => Kind::Num(v),
        Err(_) => {
            if !RE.is_match(val) {
                eprintln!("The line format is incorrectly specified: {}", val);
                exit(1);
            }
            let caps = RE.captures(val).expect("Match failed");
            Kind::Range((caps["init"].parse().unwrap(), caps["fin"].parse().unwrap()))
        }
    }
}

pub fn parse_line_ranges(vals: Vec<String>) -> Vec<Kind> {
    vals.iter().map(|x| parse_value(x)).collect()
}
