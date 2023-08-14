use std::collections::HashSet;
use itertools::Itertools;

fn parse_group(s: &str) -> HashSet<char> {
    let count = s.lines().count();
    s.chars().filter(|c| *c != '\n').counts().iter().filter(|e| *e.1 == count).map(|e| *e.0).collect()
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let answer = input.split("\n\n").map(parse_group).map(|g| g.len()).sum::<usize>();

    println!("answer {}", answer);

    Ok(())
}
