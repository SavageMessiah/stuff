use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let rows = input.lines().map(|l| l.parse::<u16>()).collect::<Result<Vec<_>, _>>()?;

    let [a1, a2] = rows.into_iter().combinations(2).find(|n| (n[0] + n[1]) == 2020 ).expect("no match")[..] else { unreachable!() };

    println!("answer {} * {} = {}", a1, a2, u32::from(a1) * u32::from(a2));

    Ok(())
}
