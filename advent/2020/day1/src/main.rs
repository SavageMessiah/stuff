use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let rows = input.lines().map(|l| l.parse::<u32>()).collect::<Result<Vec<_>, _>>()?;

    let answer = rows.into_iter().combinations(3).find(|n| n.iter().sum::<u32>() == 2020 ).expect("no match");

    println!("answer {:?} {}", answer, answer.iter().product::<u32>());

    Ok(())
}
