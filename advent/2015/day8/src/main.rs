struct Counts {
    encoded: u32,
    lit: u32,
    mem: u32,
}

fn main() {
    let mut counts = Counts{encoded: 0, lit: 0, mem: 0};
    for line in include_str!("input.txt").lines() {
        let mut iter = line.chars();
        while let Some(c) = iter.next() {
            counts.lit += 1;
            match c {
                '"' => {
                    counts.encoded += 3; // "\" or \""
                },
                '\\' => {
                    counts.mem += 1;
                    counts.encoded += 2; // \\
                    match iter.next() {
                        Some('x') => {
                            counts.encoded += 3; // xab
                            counts.lit += 3;
                            iter.next();
                            iter.next();
                        },
                        Some(_) => {
                            counts.encoded += 2; // \"
                            counts.lit += 1;
                        },
                        None => panic!("bad input")
                    }
                },
                _ => {
                    counts.encoded += 1;
                    counts.mem += 1;
                }
            }
        }
    }

    println!("{} - {} = {}", counts.lit, counts.mem, counts.lit - counts.mem);
    println!("{} - {} = {}", counts.encoded, counts.lit, counts.encoded - counts.lit);
}
