use anyhow::anyhow;
use itertools::Itertools;

fn common_bit(strs: &[&str], i: usize) -> Option<char> {
    let counts = strs.iter().map(|s| s.chars().nth(i).unwrap()).counts();
    if counts[&'0'] == counts[&'1'] {
        None
    } else if counts[&'1'] > counts[&'0'] {
        Some('1')
    } else {
        Some('0')
    }
}

fn find_match<'a, F: Fn(Option<char>) -> char>(strs: &[&'a str], f: F) -> Option<&'a str> {
    let mut prefix = "".to_string();
    let mut matches = strs.to_vec();
    for i in 0.. {
        prefix.push(f(common_bit(&matches, i)));

        matches = strs.iter().filter(|l| l.starts_with(&prefix)).copied().collect::<Vec<&str>>();
        println!("{} {}", prefix, matches.len());
        if matches.len() == 1 {
            return Some(matches[0])
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let lines = input.lines().collect::<Vec<_>>();
    let oxygen = u32::from_str_radix(find_match(&lines, |b| b.unwrap_or('1')).ok_or(anyhow!("wut"))?, 2)?;
    let scrubber = u32::from_str_radix(find_match(&lines, |b| {
        match b {
            Some('0') => '1',
            _ => '0'
        }
    }).ok_or(anyhow!("wut"))?, 2)?;

    let answer = oxygen * scrubber;

    println!("answer {}", answer);

    Ok(())
}
