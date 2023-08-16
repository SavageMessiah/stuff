use std::num::ParseIntError;
use itertools::Itertools;
use anyhow::{Result, anyhow};

fn valid(window: &[u64], val: u64) -> bool {
    window.iter().tuple_combinations().any(|(a, b)| a + b == val)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let ns = input.lines().map(|l| l.parse()).collect::<Result<Vec<u64>, ParseIntError>>()?;
    let invalid = ns.windows(25).zip(ns.iter().skip(25)).find(|(win, v)| !valid(win, **v)).unwrap().1;
    for start in 0..ns.len() {
        for end in start..=ns.len() {
            let sub = &ns[start..end];
            if sub.iter().sum::<u64>() != *invalid {
                continue;
            }
            let answer = sub.iter().max().unwrap() + sub.iter().min().unwrap();
            println!("answer: {}", answer);
            return Ok(());
        }
    }

    Err(anyhow!("no answer?"))
}
