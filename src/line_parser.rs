use std::process::exit;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum LineKind {
    Num(u32),
    Range((u32, u32)),
}

impl LineKind {
    /// Is inclusive
    pub fn contains(&self, other: u32) -> bool {
        match self {
            LineKind::Num(x) => *x == other,
            LineKind::Range((x, y)) => (*x..=*y).contains(&other),
        }
    }
}

fn _parse_value<F: FnOnce(i32)>(val: &str, f: F) -> LineKind {
    let parse_try = val.parse::<u32>();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<init>\d+)-(?P<fin>\d+)").unwrap();
    }
    match parse_try {
        Ok(v) => LineKind::Num(v),
        Err(_) => {
            if !RE.is_match(val) {
                eprintln!("The line format is incorrectly specified: {}", val);
                f(1);
            }
            let caps = RE.captures(val).expect("Match failed");
            LineKind::Range((
                caps["init"].parse::<u32>().unwrap(),
                caps["fin"].parse::<u32>().unwrap(),
            ))
        }
    }
}

fn parse_value(val: &str) -> LineKind {
    _parse_value(val, |n| exit(n))
}

pub fn parse_line_ranges(vals: Vec<String>) -> Vec<LineKind> {
    vals.iter().map(|x| parse_value(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value_with_number() {
        assert_eq!(parse_value("123"), LineKind::Num(123));
    }

    #[test]
    fn test_parse_value_with_range() {
        assert_eq!(parse_value("10-20"), LineKind::Range((10, 20)));
    }

    #[test]
    #[should_panic]
    fn test_wrong_parse() {
        let mut val = None;
        _parse_value("10-a", |x| {
            val = Some(x);
            panic!()
        });
        assert_eq!(val, Some(1))
    }

    #[test]
    #[should_panic]
    fn test_wrong_parse2() {
        let mut val = None;
        _parse_value("aa", |x| {
            val = Some(x);
            panic!()
        });
        assert_eq!(val, Some(1))
    }

    #[test]
    fn test_contains_num() {
        assert!(LineKind::Num(4).contains(4));
    }

    #[test]
    fn test_contains_range() {
        assert!(LineKind::Range((1, 10)).contains(4));
    }

    #[test]
    fn test_contains_rangeu() {
        assert!(LineKind::Range((1, 10)).contains(10));
    }

    #[test]
    fn test_contains_rangel() {
        assert!(LineKind::Range((1, 10)).contains(1));
    }
}
