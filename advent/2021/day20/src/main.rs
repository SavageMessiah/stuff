use std::{collections::HashMap, default};
use core::ops::RangeInclusive;
use itertools::{Itertools, MinMaxResult};

use anyhow::anyhow;

type Coord = [i32; 2];
type Map = HashMap<Coord, bool>;
type Algo = Vec<bool>;

fn region_idx(coord: &Coord, map: &Map, default_pixel: bool) -> usize {
    let mut idx = 0;
    for y in [-1, 0, 1] {
        for x in [-1, 0, 1]  {
            let bit = *map.get(&[coord[0] + x, coord[1] + y]).unwrap_or(&default_pixel);
            idx = (idx << 1) + bit as usize;
        }
    }
    idx
}

//copypasta from 2020 day 17
fn minmax_range(mm: MinMaxResult<i32>) -> RangeInclusive<i32> {
    match mm {
        MinMaxResult::NoElements => 0..=0,
        MinMaxResult::OneElement(e) => e..=e,
        MinMaxResult::MinMax(min, max) => min..=max
    }
}

fn bounds(map: &Map) -> [RangeInclusive<i32>; 2] {
    [minmax_range(map.keys().map(|c| c[0]).minmax()),
     minmax_range(map.keys().map(|c| c[1]).minmax())]
}

fn grow_range(range: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    (range.start() - 1)..=(range.end() + 1)
}

fn step(map: Map, algo: &Algo, default_pixel: bool) -> Map {
    let mut new = Map::new();
    for coord in bounds(&map).iter().map(grow_range).multi_cartesian_product() {
        let coord: Coord = coord.try_into().unwrap();
        let pixel = algo[region_idx(&coord, &map, default_pixel)];
        new.insert(coord, pixel);
    }
    new
}

fn run(mut map: Map, algo: &Algo, steps: usize) -> Map {
    print_map(&map);
    let mut default_pixel = false;
    for i in 1..=steps {
        println!("Step {}: {}", i, default_pixel);
        map = step(map, algo, default_pixel);
        default_pixel = if default_pixel {
            algo[511]
        } else {
            algo[0]
        };
        print_map(&map);
    }
    map
}

fn print_map(map: &Map) {
    let [xb, yb] = bounds(map);
    for y in yb.clone() {
        for x in xb.clone() {
            let c = if *map.get(&[x, y]).unwrap_or(&false) {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!("");
    }
}

fn parse_input(input: &str) -> anyhow::Result<(Algo, Map)> {
    let (algo_str, map_str) = input.split_once("\n\n").ok_or(anyhow!("missing map section"))?;

    let algo = algo_str.chars().filter(|c| !c.is_ascii_whitespace() ).map(|c| c == '#').collect();

    let mut x = 0;
    let mut y = 0;
    let mut map = Map::new();
    for c in map_str.chars() {
        match c {
            '#' => { map.insert([x, y], true); },
            '.' => { map.insert([x, y], false); },
            '\n' => {
                x = 0;
                y += 1;
                continue;
            },
            _ => return Err(anyhow!("bad char {}", c))
        }
        x += 1;
    }

    Ok((algo, map))
}

#[test]
fn test_parse_and_run() {
    let (algo, mut map) = parse_input("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###").unwrap();

    map = run(map, &algo, 2);
    assert_eq!(map.values().filter(|v| **v).count(), 35);

}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (algo, mut map) = parse_input(&input)?;
    map = run(map, &algo, 2);

    println!("{:?}", algo);
    println!("# lit pixels {}", map.values().filter(|v| **v).count());

    Ok(())
}
