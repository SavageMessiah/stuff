use anyhow::{anyhow, Result};


fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut lines = input.lines();
    let earliest_time = lines.next().ok_or(anyhow!("empty input"))?.parse::<u32>().unwrap();
    let busses = lines.next().ok_or(anyhow!("missing second line"))?.split(',').filter_map(|b| b.parse::<u32>().ok()).collect::<Vec<_>>();
    let (next_bus, time) = busses.iter().map(|b| {
        let mut mult = earliest_time / b;
        if earliest_time % b != 0 {
            mult += 1;
        }
        (*b, mult * b)
    }).min_by_key(|p| p.1).ok_or(anyhow!("no busses"))?;
    let answer = next_bus * (time - earliest_time);

    println!("answer: {:?}", answer);

    Ok(())
}
