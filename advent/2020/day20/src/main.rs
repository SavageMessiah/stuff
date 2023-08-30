use std::{fmt::Debug, str::FromStr, collections::HashMap};
use anyhow::anyhow;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Side {
    bits: u16
}

impl Side {
    fn flip(&self) -> Side {
        Side {
            bits: self.bits.reverse_bits() >> 6
        }
    }
}

impl Debug for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#012b}", self.bits))
    }
}


impl FromStr for Side {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = 0;
        if s.len() > 16 {
            return Err(anyhow!("too long"));
        }
        for c in s.chars() {
            bits = bits << 1;
            bits += (c == '#') as u16;
        }
        Ok(Side {
            bits
        })
    }
}

enum Transform {
    L, R, H, V
}

#[derive(Debug)]
struct Tile {
    id: u32,
    sides: [Side; 4]
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    let mut tiles = vec![];
    for t in input.split("\n\n") {
        if t.is_empty() {
            continue;
        }
        let id = t.lines().nth(0).unwrap().split(' ').nth(1).unwrap().strip_suffix(":").unwrap().parse().unwrap(); //lol
        let top = t.lines().nth(1).unwrap();
        let bottom = t.lines().nth(10).unwrap();
        let mut left = "".to_string();
        let mut right = "".to_string();

        for l in t.lines().skip(1) {
            left.push(l.chars().nth(0).unwrap());
            right.push(l.chars().last().unwrap())
        }

        tiles.push(Tile {
            id,
            sides: [top.parse().unwrap(), right.parse().unwrap(), bottom.parse().unwrap(), left.parse().unwrap()]
        })
    }
    tiles
}

fn quick_match(tiles: &[Tile]) -> HashMap<Side, Vec<u32>> {
    let mut map = HashMap::new();
    for t in tiles {
        for s in t.sides {
            for s in [s, s.flip()] {
                map.entry(s).or_insert(vec![]).push(t.id);
            }
        }
    }

    map
}

//#.#.
//.##.
//###.
//.#.#
//
// R
// .#.#
// ###.
// .###
// #...
//
// RR
// #.#.
// .###
// .##.
// .#.#
//
// L
// ...#
// ###.
// .###
// #.#.
//
// H
// .#.#
// .##.
// .###
// #.#.
//
// V
// .#.#
// ###.
// .##.
// #.#.
//
// HV
// #.#.
// .###
// .##.
// .#.#
//
//48 sides are unmatched
//264 sides are matched

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let tiles = parse_tiles(&input);
    let m = quick_match(&tiles);
    let counts = m.iter().counts_by(|(k, v)| v.len());

    let mut answer = 1u64;
    for t in &tiles {
        let matches = t.sides.iter().filter(|s| {
            m.get(s).unwrap().len() == 1 && m.get(&s.flip()).unwrap().len() == 1
        }).count();
        if matches == 2 {
            println!("{:?}", t);
            answer *= t.id as u64;
        }
    }

    println!("answer: {:?}", answer);
}
