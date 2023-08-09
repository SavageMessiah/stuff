use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let rows = input.lines().map(|l| l.parse::<u16>()).collect::<Result<Vec<_>, _>>()?;

    let answer = rows.into_iter().combinations(3).find(|n| n.iter().sum::<u16>() == 2020 ).expect("no match");

    println!("answer {:?} {}", answer, answer.iter().map(Clone::clone).map(u32::from).product::<u32>());

    Ok(())
}
