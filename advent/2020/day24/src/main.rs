use std::collections::HashSet;

use itertools::{Itertools, MinMaxResult};

type Coord = (i32, i32);

type Map = HashSet<Coord>;

#[derive(Clone, Copy, Debug)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE
}

fn flipped(map: &Map) -> usize {
    map.len()
}

fn shift(coord: Coord, dir: Direction) -> Coord {
    let (mut x, mut y) = coord;
    use Direction::*;
    match dir {
        E => x = x + 1,
        SE => {
            y = y + 1;
        },
        SW => {
            x = x - 1;
            y = y + 1;
        },
        W => x = x - 1,
        NW => {
            y = y - 1;
        },
        NE => {
            x = x + 1;
            y = y - 1;
        }
    }
    (x, y)
}

fn step(map: &Map) -> Map {
    let mut next = map.clone();
    let MinMaxResult::MinMax(xmin, xmax) = map.iter().map(|coord| coord.0).minmax() else { unreachable!() };
    let MinMaxResult::MinMax(ymin, ymax) = map.iter().map(|coord| coord.1).minmax() else { unreachable!() };
    for y in ymin-1..=ymax+1 {
        for x in xmin-1..=xmax+1 {
            let coord = (x, y);
            use Direction::*;
            let adjacent_flipped = [E, SE, SW, W, NW, NE].iter()
                                                         .map(move |dir| shift(coord, *dir))
                                                         .filter(|coord| map.contains(coord))
                                                         .count();
            if map.contains(&coord) {
                if adjacent_flipped == 0 || adjacent_flipped > 2 {
                    next.remove(&coord);
                }
            } else {
                if adjacent_flipped == 2 {
                    next.insert(coord);
                }
            }
        }
    }
    next
}

fn flip(map: &mut Map, dirs: &[Direction]) {
    let mut coord = (0, 0);
    for dir in dirs {
        //println!("at {}, {}", x, y);
        coord = shift(coord, *dir);
        //println!("moved {:?} to {}, {}", dir, x, y);
    }
    if map.contains(&coord) {
        //println!("flipping back");
        map.remove(&coord);
    } else {
        //println!("flipping");
        map.insert(coord);
    }
}

fn parse_dirs(s: &str) -> Vec<Direction> {
    let mut dirs = vec![];
    let mut iter = s.chars();
    while let Some(c) = iter.next() {
        use Direction::*;
        match c {
            'e' => dirs.push(E),
            'w' => dirs.push(W),
            's' => match iter.next() {
                Some('e') => dirs.push(SE),
                Some('w') => dirs.push(SW),
                _ => panic!("bad dir input")
            },
            'n' => match iter.next() {
                Some('e') => dirs.push(NE),
                Some('w') => dirs.push(NW),
                _ => panic!("bad dir input")
            },
            _ => panic!("bad dir input")
        }
    }
    dirs
}

fn parse_input(s: &str) -> Vec<Vec<Direction>> {
    s.lines().map(parse_dirs).collect()
}

fn parse_flip_count(s: &str, steps: usize) -> usize {
    let all_dirs = parse_input(s);
    let mut map = Map::new();
    for dirs in all_dirs {
        flip(&mut map, &dirs);
    }
    for i in 0..steps {
        println!("Day {}: {}", i, flipped(&map));
        map = step(&map);
    }
    flipped(&map)
}

#[test]
fn test_flip() {
    assert_eq!(parse_flip_count("sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew", 10), 37);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", parse_flip_count(&input, 100));
}
