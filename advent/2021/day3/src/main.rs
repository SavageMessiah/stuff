fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut counts = [[0; 2]; 12];
    for l in input.lines() {
        for (i, c) in l.chars().enumerate() {
            let bit = if c == '1' { 1 } else { 0 };
            counts[i][bit] += 1;
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;

    for bit in counts {
        let (most, least) = if bit[0] < bit[1] {
            (1, 0)
        } else {
            (0, 1)
        };
        println!("{:?} {} {}", bit, most, least);
        gamma = (gamma << 1) + most;
        epsilon = (epsilon << 1) + least;
    }

    let answer = gamma * epsilon;

    println!("answer {}", answer);

    Ok(())
}
