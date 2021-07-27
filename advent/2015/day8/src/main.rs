struct Counts {
    lit: u32,
    mem: u32,
}

struct State(fn(char, &mut Counts) -> State);

fn escape(c: char, counts: &mut Counts) -> State {
    match c {
        'x' => {
            counts.lit += 1;
            return State(|_, counts| { //first hex
                counts.lit += 1;
                return State(|_, counts| { //second hex
                    counts.lit += 1;
                    counts.mem += 1;
                    return State(normal);
                })
            })
        },
        _ => {
            counts.lit += 1;
            counts.mem += 1;
            return State(normal);
        },
    }
}

fn normal(c: char, counts: &mut Counts) -> State {
    match c {
        '"' => {
            counts.lit += 1;
            return State(normal);
        },
        '\\' => {
            counts.lit += 1;
            return State(escape);
        },
        _ => {
            counts.lit += 1;
            counts.mem += 1;
            return State(normal);
        },
    }
}

fn main() {
    let mut counts = Counts{lit: 0, mem: 0};
    for line in include_str!("input.txt").lines() {
        let mut s = State(normal);
        for c in line.chars() {
            s = s.0(c, &mut counts);
        }
    }
    println!("{} - {} = {}", counts.lit, counts.mem, counts.lit - counts.mem);
}
