use std::fmt::Display;

use anyhow::Result;
use itertools::Itertools;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Floor,
    Empty,
    Occupied
}

#[derive(Clone, PartialEq, Eq)]
struct Grid {
    width: usize,
    tiles: Vec<Tile>
}

static DIRS: [isize; 3] = [1, 0, -1];

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, t) in self.tiles.iter().enumerate() {
            if i % self.width == 0 {
                f.write_str("\n")?;
            }
            f.write_str(match t {
                Tile::Floor => ".",
                Tile::Empty => "L",
                Tile::Occupied => "#"
            })?;
        }
        Ok(())
    }
}

impl Grid {
    fn parse(s: &str) -> Grid {
        let mut tiles = Vec::new();
        let mut width: Option<usize> = None;
        for c in s.chars() {
            match c {
                '.' => tiles.push(Tile::Floor),
                'L' => tiles.push(Tile::Empty),
                '#' => tiles.push(Tile::Occupied),
                '\n' => if width == None {
                    width = Some(tiles.len())
                },
                _ => unreachable!("wat")
            }
        }
        Grid {
            width: width.unwrap(),
            tiles
        }
    }

    fn dims(&self) -> (usize, usize) {
        (self.width, self.tiles.len() / self.width)
    }

    fn neighbors(&self, i: usize) -> impl Iterator<Item = &Tile> {
        let (w, h) = self.dims();
        let x = i % w;
        let y = i / w;

        DIRS.iter().cartesian_product(DIRS.iter()).filter_map(move |(dx, dy)| {
            //skip self
            if *dx == 0 && *dy == 0 {
                return None;
            }
            //clip to grid bounds
            if (x == 0 && *dx == -1) || (x == w - 1 && *dx == 1) || (y == 0 && *dy == -1) || (y == h - 1 && *dy == 1) {
                return None;
            }

            let i = (x as isize + dx) + (y as isize + dy) * w as isize;
            Some(&self.tiles[i as usize])
        })
    }

    fn step(&self) -> Grid {
        let mut next = self.clone();
        for (i, t) in next.tiles.iter_mut().enumerate() {
            let cur = self.tiles[i];
            if cur == Tile::Floor {
                continue;
            }

            let occupied = self.neighbors(i).filter(|n| **n == Tile::Occupied).count();
            //println!("n: {} occupied: {}", self.neighbors(i).count(), occupied);
            *t = match occupied {
                0 => Tile::Occupied,
                n if n >= 4 => Tile::Empty,
                _ => cur
            }
        }
        next
    }

    fn run(&self) -> Grid {
        let mut prev = self.clone();
        let mut n = 0;
        loop {
            println!("running generation {}", n);
            let next = prev.step();
            //println!("{}", next);
            if next == prev {
                return next;
            }
            prev = next;
            n += 1;
        }
    }
}

#[test]
fn test_run() {
    let start = Grid::parse("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL");
    println!("{}", start);
    let end = start.run();
    let occupied = end.tiles.iter().filter(|t| **t == Tile::Occupied).count();
    assert_eq!(occupied, 37);
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let start = Grid::parse(&input);
    let end = start.run();
    let answer = end.tiles.iter().filter(|t| **t == Tile::Occupied).count();

    println!("answer: {:?}", answer);

    Ok(())
}
