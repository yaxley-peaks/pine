#[derive(Debug)]
pub enum Kind {
    Num(u32),
    Range((u32, u32)),
}

fn parse_value(val: &str) -> Result<Kind, String> {
    let parse_try = val.parse::<u32>();
    // Move to regex based parsing
    match parse_try {
        Ok(v) => Ok(Kind::Num(v)),
        Err(_) => {
            todo!()
        }
    }
}
