use std::num::ParseIntError;

use itertools::{MinMaxResult, Itertools};

fn parse_input(s: &str) -> Result<Vec<i32>, ParseIntError> {
    s.split(',').map(|n| n.trim_end().parse() ).collect()
}

fn triangle(mut n: i32) -> i32 {
    let mut t = n;
    while n != 0 {
        n -= 1;
        t += n;
    }
    t
}

fn fuel_usage(crabs: &[i32], target: i32) -> i32 {
    crabs.iter().map(|c| triangle((c - target).abs()) ).sum()
}

fn optimal_position(crabs: &[i32]) -> i32 {
    let MinMaxResult::MinMax(min, max) = crabs.iter().minmax() else { unreachable!() };
    (*min..=*max).min_by_key(|target| fuel_usage(crabs, *target) ).unwrap()
}

#[test]
fn test_fuel_usage() {
    let crabs = vec![16,1,2,0,4,2,7,1,2,14];

    assert_eq!(fuel_usage(&crabs, 5), 168);
    assert_eq!(fuel_usage(&crabs, 2), 206);
}

#[test]
fn test_optimal_position() {
    let crabs = vec![16,1,2,0,4,2,7,1,2,14];

    assert_eq!(optimal_position(&crabs), 5);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let crabs = parse_input(&input)?;
    let target = optimal_position(&crabs);

    println!("{}", fuel_usage(&crabs, target));

    Ok(())
}
