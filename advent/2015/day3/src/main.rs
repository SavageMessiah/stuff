use std::collections::HashMap;

type Coord = (i32, i32);

fn scoot(c: Coord, dir: char) -> Coord {
    match dir {
        '^' => (c.0, c.1 + 1),
        'v' => (c.0, c.1 - 1),
        '<' => (c.0 - 1, c.1),
        '>' => (c.0 + 1, c.1),
        _ => (c.0, c.1),
    }
}

fn main() -> anyhow::Result<()> {
    let mut g : HashMap<Coord, u32> = HashMap::new();
    let mut santas = [(0, 0), (0, 0)];
    g.insert(santas[0], 2);

    for dir in include_str!("input.txt").chars() {
        santas[0] = scoot(santas[0], dir);
        *g.entry(santas[0]).or_insert(0) += 1;
        santas.reverse();
    }

    println!("At least 1 : {}", g.values().len());

    Ok(())
}
