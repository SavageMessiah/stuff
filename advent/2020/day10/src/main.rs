use std::num::ParseIntError;
use itertools::Itertools;
use anyhow::Result;


fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut adapters = input.lines().map(|l| l.parse()).collect::<Result<Vec<u32>, ParseIntError>>()?;
    let max = adapters.iter().max().unwrap() + 3;
    adapters.push(0);
    adapters.push(max);
    adapters.sort();
    let counts = adapters.iter().tuple_windows().map(|(a, b)| b - a).counts();
    let answer = counts[&1] * counts[&3];

    println!("answer: {:?}", answer);

    Ok(())
}
