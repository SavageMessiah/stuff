use std::{str::FromStr, collections::HashMap, fmt::Display};

use anyhow::anyhow;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u32,
    y: u32
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(anyhow!("bad point def: {}", s))?;
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Line {
    start: Point,
    end: Point
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").ok_or(anyhow!("bad line def: {}", s))?;
        Ok(Line {
            start: start.parse()?,
            end: end.parse()?
        })
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} -> {}", self.start, self.end))
    }
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }
}

type Map = HashMap<Point, u32>;

fn mark_point(map: &mut Map, p: Point) {
    *map.entry(p).or_insert(0) += 1;
}

fn towards_end(n: u32, end: u32) -> u32 {
    use std::cmp::Ordering::*;
    match n.cmp(&end) {
        Less => n + 1,
        Equal => n,
        Greater => n - 1,
    }
}

fn mark_line(map: &mut Map, l: &Line) {
    println!("marking line: {}", l);
    let mut point = l.start;
    while point != l.end {
        println!("marking point: {}", point);
        mark_point(map, point);
        point.x = towards_end(point.x, l.end.x);
        point.y = towards_end(point.y, l.end.y);
    }
    println!("marking end point: {}", point);
    mark_point(map, point);
}

fn find_danger(lines: &[Line]) -> usize {
    let mut map = Map::new();
    for l in lines {
        mark_line(&mut map, l);
    }
    map.values().filter(|p| **p > 1).count()
}


fn parse_input(s: &str) -> anyhow::Result<Vec<Line>> {
    s.lines().map(|l| l.parse() ).collect()
}

#[test]
fn test_score() {
    let lines = parse_input("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2").unwrap();

    assert_eq!(find_danger(&lines), 12);
}


fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let lines = parse_input(&input)?;

    println!("{}", find_danger(&lines));

    Ok(())
}
