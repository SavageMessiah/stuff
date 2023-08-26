use std::{iter::repeat, collections::HashSet, ops::RangeInclusive, convert::TryInto};

use anyhow::Result;
use itertools::{Itertools, MinMaxResult};

type Coord = [i32; 3];


fn neighbors(coord: &Coord) -> impl Iterator<Item = Coord> + '_ {
    repeat([1, 0, -1]).take(3).multi_cartesian_product().filter_map(move |v| {
        if v[0] == 0 && v[1] == 0 && v[2] == 0 {
            None
        } else {
            Some([coord[0] + v[0], coord[1] + v[1], coord[2] + v[2]])
        }
    })
}

type Grid = HashSet<Coord>;

fn count_active_neighbors(grid: &Grid, coord: &Coord) -> usize {
    neighbors(coord).filter(|c| grid.contains(c)).count()
}

fn minmax_range(mm: MinMaxResult<i32>) -> RangeInclusive<i32> {
    match mm {
        MinMaxResult::NoElements => 0..=0,
        MinMaxResult::OneElement(e) => e..=e,
        MinMaxResult::MinMax(min, max) => min..=max
    }
}

fn bounds(grid: &Grid) -> [RangeInclusive<i32>; 3] {
    [minmax_range(grid.iter().map(|c| c[0]).minmax()),
     minmax_range(grid.iter().map(|c| c[1]).minmax()),
     minmax_range(grid.iter().map(|c| c[2]).minmax())]
}

fn grow_range(range: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    (range.start() - 1)..=(range.end() + 1)
}

fn cycle(grid: Grid) -> Grid {
    let mut new = grid.clone();

    for coord in bounds(&grid).iter().map(|r| grow_range(r)).multi_cartesian_product() {
        let coord: Coord = coord.try_into().unwrap();
        let active = grid.contains(&coord);

        let active_neighbors = count_active_neighbors(&grid, &coord);
        println!("{:?} {} {}", coord, active, active_neighbors);
        if active && !(active_neighbors == 2 || active_neighbors == 3) {
            println!("going inactive");
            new.remove(&coord);
        }
        if !active && active_neighbors == 3 {
            println!("going active");
            new.insert(coord);
        }

    }

    new
}

fn parse_grid(s: &str) -> Grid {
    let mut g = Grid::new();
    let mut y = 0;
    for l in s.lines() {
        let mut x = 0;
        for c in l.chars() {
            if c == '#' {
                g.insert([x, y, 0]);
            }
            x += 1;
        }
        y += 1;
    }
    g
}

fn count_active(grid: &Grid) -> usize {
    grid.len()
}

fn print_grid(grid: &Grid) {
    let [xb, yb, zb] = bounds(grid);
    for z in zb.clone() {
        println!("\nz={}", z);
        for y in yb.clone() {
            for x in xb.clone() {
                let c = if grid.contains(&[x, y, z]) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!("");
        }

    }
}

#[test]
fn test_cycle() {
    let mut grid = parse_grid(
".#.
..#
###");
    print_grid(&grid);
    assert_eq!(count_active(&grid), 5);
    for i in 1..=6 {
        println!("cycle {}", i);
        grid = cycle(grid);
        print_grid(&grid);
    }
    assert_eq!(count_active(&grid), 112);
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut grid = parse_grid(&input);
    for i in 1..=6 {
        println!("running cycle {}, {} are active", i, count_active(&grid));
        grid = cycle(grid);
    }
    let answer = count_active(&grid);

    println!("answer: {:?}", answer);

    Ok(())
}
