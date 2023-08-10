#[derive(Debug)]
struct Map {
    trees: Vec<u32>,
}

impl Map {
    fn new() -> Map {
        Map {
            trees: Vec::new()
        }
    }

    fn count_trees(&self, x: usize, _y: usize) -> usize {
        let mut trees = 0;
        let mut xpos: usize = 0;
        let mut ypos: usize = 0;
        for row in self.trees.iter() {
            let shift = 31 - xpos;
            println!("r: {:#034b} x: {} y: {}\nm: {:#034b} shift: {} match: {}\n", row, xpos, ypos, 1 << shift, shift, row & (1 << shift));
            if (row & (1 << shift)) != 0 {
                trees += 1;
            }

            xpos = (xpos + x) % 31; //each row is actually 31 wide, not 32
            ypos = 0;
        }

        trees
    }

    fn add_row(&mut self, line: &str) {
        let mut row: u32 = 0;
        let mut bit: u32 = 1 << 31;
        for c in line.chars() {
            if c == '#' {
                row |= bit;
            }
            bit >>= 1;
        }
        self.trees.push(row);
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut map = Map::new();
    for line in input.lines() {
        map.add_row(line);
    }
    let tree_count = map.count_trees(3, 1);

    println!("tree count {}", tree_count);

    Ok(())
}
