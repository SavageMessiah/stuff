use itertools::Itertools;
use std::num::ParseIntError;

type Fish = [u64; 9];

fn parse_input(s: &str) -> Result<Fish, ParseIntError> {
    let all_fish = s.split(',').map(|n| n.trim_end().parse() ).collect::<Result<Vec<usize>, _>>()?;
    let counts = all_fish.iter().counts();
    let mut fish = [0u64; 9];
    for i in 0..fish.len() {
        fish[i] = *counts.get(&i).unwrap_or(&0usize) as u64;
    }
    Ok(fish)
}

fn step(fish: &mut Fish) {
    fish.rotate_left(1);
    fish[6] += fish[8];
}

fn count_fish(fish: &Fish) -> u64 {
    fish.iter().sum()
}

fn run(initial_fish: &Fish, steps: u32) -> u64 {
    let mut fish = initial_fish.clone();
    println!("initial fish: {} {:?}", count_fish(&fish), fish);
    for i in 1..=steps {
        step(&mut fish);
        println!("after {} days: {} {:?}", i, count_fish(&fish), fish);
    }

    count_fish(&fish)
}

#[test]
fn test_score() {
    let fish = parse_input("3,4,3,1,2").unwrap();
    println!("{:?}", fish);

    assert_eq!(run(&fish, 18), 26);
    assert_eq!(run(&fish, 80), 5934);
    assert_eq!(run(&fish, 256), 26984457539);
}


fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let fish = parse_input(&input)?;

    println!("{}", run(&fish, 256));

    Ok(())
}
