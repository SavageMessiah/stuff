use std::collections::HashSet;
use itertools::Itertools;

fn parse(s: &str) -> u32 {
    let mut row: u32 = 0;
    let mut col: u32 = 0;
    for c in s.chars() {
        match c {
            'F' => row = row << 1,
            'B' => row = (row << 1) | 1,
            'L' => col = col << 1,
            'R' => col = (col << 1) | 1,
            _ => unreachable!("bad input")
        }
    }

    let id = row * 8 + col;

    println!("s: {} r: {:b} c: {:b} id: {}", s, row, col, id);
    id
}

#[test]
fn test_parse() {
    assert_eq!(parse("BFFFBBFRRR"), 567);
    assert_eq!(parse("FFFBBBFRRR"), 119);
    assert_eq!(parse("BBFFBBFRLL"), 820);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let seats = input.lines().map(parse).collect::<HashSet<_>>();
    let answer = (0..128).
        into_iter().
        cartesian_product(0..8)
        .map(|(r, c)| r * 8 + c)
        .find(|seat| !seats.contains(seat) && seats.contains(&(seat + 1)) && seats.contains(&(seat - 1)))
        .expect("no answer");

    println!("answer {}", answer);

    Ok(())
}
