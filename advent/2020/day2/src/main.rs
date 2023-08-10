use anyhow::{anyhow, Result};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref ROW: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): (.*)").expect("bad regex");
}

struct Entry {
    lower: usize,
    upper: usize,
    letter: char,
    pw: String
}

fn parse(row: &str) -> Result<Entry> {
    let captures = ROW.captures(row).ok_or(anyhow!("bad row: {}", row))?;

    Ok(Entry {
        lower: captures[1].parse()?,
        upper: captures[2].parse()?,
        letter: captures[3].chars().next().unwrap(), //wouldn't match if this was empty
        pw: captures[4].to_string(),
    })
}

fn valid(e: &Entry) -> bool {
    let count = e.pw.chars().filter(|c| *c == e.letter).count();
    count >= e.lower && count <= e.upper
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let parsed = input.lines().map(parse).collect::<Result<Vec<_>, _>>()?;
    let valid = parsed.into_iter().filter(valid).collect::<Vec<_>>();

    println!("valid count {}", valid.len());

    Ok(())
}
