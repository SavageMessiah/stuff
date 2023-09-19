use anyhow::anyhow;
use itertools::Itertools;

struct Octopus {
    energy: u8,
    flashed: bool
}

impl Octopus {
    fn reset(&mut self) {
        if self.flashed {
            self.energy = 0;
            self.flashed = false;
        }
    }
}

type Octogrid = Vec<Vec<Octopus>>;

fn flash(grid: &mut Octogrid, x: i32, y: i32) -> u32 {
    let octopus = grid.get_mut(y as usize).unwrap().get_mut(x as usize).unwrap();
    if octopus.flashed || octopus.energy <= 9 {
        return 0;
    }

    octopus.flashed = true;

    let mut flashed = 1;

    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    for (dy, dx) in [1, 0, -1].iter().cartesian_product([1, 0, -1]) {
        //skip self
        if dx == 0 && *dy == 0 {
            continue;
        }
        //clip to grid bounds
        if (x == 0 && dx == -1) || (x == w - 1 && dx == 1) || (y == 0 && *dy == -1) || (y == h - 1 && *dy == 1) {
            continue;
        }

        let ny = y + dy;
        let nx = x + dx;
        grid[ny as usize][nx as usize].energy += 1;
        flashed += flash(grid, nx, ny);
    }

    flashed
}

fn step(grid: &mut Octogrid) -> u32 {
    for row in &mut *grid {
        for octopus in row {
            octopus.energy += 1;
        }
    }

    let mut flashed = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            flashed += flash(grid, x as i32, y as i32);
        }
    }

    for row in &mut *grid {
        for octopus in row {
            octopus.reset()
        }
    }

    for row in grid {
        for octopus in row {
            print!("{} ", octopus.energy);
        }
        println!("");
    }

    flashed
}

fn steps(grid: &mut Octogrid, n: u32) -> u32 {
    let mut total_flashed = 0;

    for row in &*grid {
        for octopus in row {
            print!("{}", octopus.energy);
        }
        println!("");
    }
    println!("\n");
    for i in 1..=n {
        let flashed = step(grid);
        println!("step {}: {} flashed", i, flashed);
        total_flashed += flashed;
    }
    total_flashed
}

fn parse_input(input: &str) -> anyhow::Result<Octogrid> {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
           let energy = c.to_digit(10).ok_or(anyhow!("bad energy {}", c))? as u8;
            row.push(Octopus { energy, flashed: false })
        }
        grid.push(row);
    }
    Ok(grid)
}

#[test]
fn test_parse_and_answer() {
    let mut grid = parse_input("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526").unwrap();

    assert_eq!(steps(&mut grid, 10), 204);
    assert_eq!(steps(&mut grid, 90), 1656 - 204);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut grid = parse_input(&input)?;

    println!("{}", steps(&mut grid, 100));

    Ok(())
}
