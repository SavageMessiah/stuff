use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let depths = input.lines()
                      .map(|l| l.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    let answer = depths.windows(3).map(|w| w.iter().sum::<u32>()).tuple_windows().filter(|(a, b)| b > a).count();

    println!("answer {}", answer);

    Ok(())
}
