use std::num::ParseIntError;
use itertools::Itertools;
use anyhow::Result;

fn valid(window: &[u64], val: u64) -> bool {
    window.iter().tuple_combinations().any(|(a, b)| a + b == val)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let ns = input.lines().map(|l| l.parse()).collect::<Result<Vec<u64>, ParseIntError>>()?;
    let answer = ns.windows(25).zip(ns.iter().skip(25)).find(|(win, v)| !valid(win, **v)).unwrap().1;

    println!("answer: {}", answer);
    Ok(())
}
