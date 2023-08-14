use std::collections::HashSet;
use itertools::Itertools;

fn parse_group(s: &str) -> HashSet<char> {
    s.chars().filter(|c| *c != '\n').collect()
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let answer = input.split("\n\n").map(parse_group).map(|g| g.len()).sum::<usize>();

    println!("answer {}", answer);

    Ok(())
}
