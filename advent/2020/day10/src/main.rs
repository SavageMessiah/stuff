use std::{cmp::min, num::ParseIntError, collections::HashMap};
use anyhow::Result;

fn count_chains(rest: &[u32], memo: &mut HashMap<u32, u64>) -> u64 {
    if rest.len() == 1 {
        return 1;
    }

    let mut total = 0;
    let this = rest[0];
    for i in 1..min(4, rest.len()) {
        let next = rest[i];
        if next - this > 3 {
            break;
        }
        total += if let Some(n) = memo.get(&next) {
            *n
        } else {
            let n = count_chains(&rest[i..], memo);
            memo.insert(next, n);
            n
        }
    }
    total
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut adapters = input.lines().map(|l| l.parse()).collect::<Result<Vec<u32>, ParseIntError>>()?;
    let max = adapters.iter().max().unwrap() + 3;
    adapters.push(0);
    adapters.push(max);
    adapters.sort();
    let mut memo = HashMap::new();
    let answer = count_chains(&adapters, &mut memo);

    println!("answer: {:?}", answer);

    Ok(())
}
