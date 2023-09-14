use std::{num::ParseIntError, iter::repeat};

fn parse_input(s: &str) -> Result<Vec<u8>, ParseIntError> {
    s.split(',').map(|n| n.trim_end().parse() ).collect()
}

fn step(fish: &mut Vec<u8>) {
    let mut spawned = 0;
    for f in &mut *fish {
        if *f == 0 {
            *f = 6;
            spawned += 1;
        } else {
            *f -= 1;
        }
    }
    fish.reserve(spawned);
    fish.extend(repeat(8).take(spawned));
}

fn run(initial_fish: &Vec<u8>, steps: u32) -> usize {
    let mut fish = initial_fish.clone();
    println!("initial fish: {}", fish.len());
    for i in 1..=steps {
        step(&mut fish);
        println!("after {} days: {}", i, fish.len());
    }

    fish.len()
}

#[test]
fn test_score() {
    let fish = parse_input("3,4,3,1,2").unwrap();

    assert_eq!(run(&fish, 18), 26);
    assert_eq!(run(&fish, 80), 5934);
}


fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let fish = parse_input(&input)?;

    println!("{}", run(&fish, 80));

    Ok(())
}
