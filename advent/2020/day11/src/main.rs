use std::{fmt::Display, iter::successors};

use anyhow::Result;
use itertools::Itertools;

type Coord = (usize, usize);
type Diff = (isize, isize);

fn shift(this: Coord, d: Diff, bounds: Coord) -> Option<Coord> {
    if (this.0 == 0 && d.0 == -1) || (this.0 == bounds.0 - 1 && d.0 == 1) ||
        (this.1 == 0 && d.1 == -1) || (this.1 == bounds.1 - 1 && d.1 == 1) {
            return None;
        }
    Some(((this.0 as isize + d.0) as usize, (this.1 as isize + d.1) as usize))
}

fn line(this: Coord, d: Diff, bounds: Coord) -> impl Iterator<Item = Coord> {
    successors(Some(this), move |c| shift(*c, d, bounds)).skip(1)
}

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

    fn dims(&self) -> Coord {
        (self.width, self.tiles.len() / self.width)
    }

    fn get(&self, c: Coord) -> &Tile {
        let dims = self.dims();
        let i = c.0 + c.1 * dims.0;
        &self.tiles[i]
    }

    fn seen_seats(&self, i: usize) -> impl Iterator<Item = &Tile> {
        let dims = self.dims();
        let pos = (i % self.width, i / self.width);
        DIRS.iter().copied().cartesian_product(DIRS.iter().copied()).filter(|d| *d != (0, 0)).filter_map(move |d| {
            //println!("pos: {:?}, d: {:?}, l: {}", pos, d, line(pos, d, dims).count());
            line(pos, d, dims).find(|c| *self.get(*c) != Tile::Floor)
        }).map(move |c| self.get(c))
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

            let occupied = self.seen_seats(i).filter(|n| **n == Tile::Occupied).count();
            //println!("n: {} occupied: {}", self.seen_seats(i).count(), occupied);
            *t = match occupied {
                0 => Tile::Occupied,
                n if n >= 5 => Tile::Empty,
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
    assert_eq!(occupied, 26);
}

#[test]
fn test_corner() {
    let mut it = line((7,7), (1,1), (8, 8));
    assert_eq!(it.next(), None);
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    //let dims = (8, 8);
    //let pos = (4, 4);
    //for d in DIRS.iter().copied().cartesian_product(DIRS.iter().copied()).filter(|d| *d != (0, 0)) {
    //    println!("pos: {:?} d: {:?}", pos, d);
    //    println!("{:?}", line(pos, d, dims).collect::<Vec<_>>());

    //}
    let start = Grid::parse(&input);
    let end = start.run();
    let answer = end.tiles.iter().filter(|t| **t == Tile::Occupied).count();

    println!("answer: {:?}", answer);

    Ok(())
}
