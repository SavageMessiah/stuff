struct Counts {
    encoded: u32,
    lit: u32,
    mem: u32,
}

//bah, wish this could be type State = fn(char, &mut Counts) -> State
struct State(fn(char, &mut Counts) -> State);

fn escape(c: char, counts: &mut Counts) -> State {
    match c {
        'x' => {
            counts.encoded += 1;
            counts.lit += 1;
            return State(|_, counts| { //first hex
                counts.encoded += 1;
                counts.lit += 1;
                return State(|_, counts| { //second hex
                    counts.encoded += 1;
                    counts.lit += 1;
                    counts.mem += 1;
                    return State(normal);
                })
            })
        },
        _ => {
            counts.encoded += 2; // \_
            counts.lit += 1;
            counts.mem += 1;
            return State(normal);
        },
    }
}

fn normal(c: char, counts: &mut Counts) -> State {
    match c {
        '"' => {
            counts.encoded += 3; // "\" or \""
            counts.lit += 1;
            return State(normal);
        },
        '\\' => {
            counts.encoded += 2; // \\
            counts.lit += 1;
            return State(escape);
        },
        _ => {
            counts.encoded += 1;
            counts.lit += 1;
            counts.mem += 1;
            return State(normal);
        },
    }
}

fn main() {
    let mut counts = Counts{encoded: 0, lit: 0, mem: 0};
    for line in include_str!("input.txt").lines() {
        let mut s = State(normal);
        for c in line.chars() {
            s = s.0(c, &mut counts);
        }
    }

    println!("{} - {} = {}", counts.lit, counts.mem, counts.lit - counts.mem);
    println!("{} - {} = {}", counts.encoded, counts.lit, counts.encoded - counts.lit);
}
