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
    let answer = input.lines().map(parse).max().unwrap();

    println!("answer {}", answer);

    Ok(())
}
