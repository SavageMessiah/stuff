use std::num::ParseIntError;

fn parse_input(s: &str) -> Result<Vec<i32>, ParseIntError> {
    s.split(',').map(|n| n.trim_end().parse() ).collect()
}

fn fuel_usage(crabs: &[i32], target: i32) -> i32 {
    crabs.iter().map(|c| (c - target).abs() ).sum()
}

fn median_int(ints: &[i32]) -> i32 {
    let mut ints = ints.to_owned();
    ints.sort();
    let mid = ints.len() / 2;

    if ints.len() % 2 == 0 {
        (ints[mid - 1] + ints[mid]) / 2
    } else {
        ints[mid]
    }
}

fn optimal_position(crabs: &[i32]) -> i32 {
    median_int(crabs)
}

#[test]
fn test_fuel_usage() {
    let crabs = vec![16,1,2,0,4,2,7,1,2,14];

    assert_eq!(fuel_usage(&crabs, 1), 41);
    assert_eq!(fuel_usage(&crabs, 2), 37);
    assert_eq!(fuel_usage(&crabs, 3), 39);
}

#[test]
fn test_optimal_position() {
    let crabs = vec![16,1,2,0,4,2,7,1,2,14];

    assert_eq!(optimal_position(&crabs), 2);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let crabs = parse_input(&input)?;
    let target = optimal_position(&crabs);

    println!("{}", fuel_usage(&crabs, target));

    Ok(())
}
