fn char_to_dir(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let floor: i32 = input.chars().map(char_to_dir).sum();

    let basement_idx = input.chars().map(char_to_dir).scan(0, |floor, dir| {
        *floor += dir;
        Some(*floor)
    }).enumerate().find(|x| x.1 == -1).expect("Basement never reached").0 + 1;

    println!("Final Floor: {} Basement Reached at: {}", floor, basement_idx);

    Ok(())
}
