use crate::line_parser::LineKind;

#[derive(Debug)]
enum Marker {
    Keep,
    Dont,
}

#[derive(Debug)]
pub struct MarkedLine {
    index: u32,
    marker: Marker,
    line: String,
}
fn ranges_contain(rngs: &Vec<LineKind>, other: u32) -> bool {
    rngs.into_iter().any(|x| x.contains(other))
}

pub fn mark_lines(kinds: Vec<LineKind>, lines: Vec<String>) -> Vec<MarkedLine> {
    let mut res = Vec::new();
    for (i, l) in lines.into_iter().enumerate() {
        let i = i.try_into().unwrap();
        if ranges_contain(&kinds, i) {
            res.push(MarkedLine {
                index: i,
                marker: Marker::Keep,
                line: l.to_string(),
            });
        } else {
            res.push(MarkedLine {
                index: i,
                marker: Marker::Dont,
                line: l.to_string(),
            });
        }
    }
    res
}
