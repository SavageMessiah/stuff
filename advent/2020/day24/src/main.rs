use std::collections::HashSet;

type Coord = (i32, i32);

type Map = HashSet<Coord>;

#[derive(Debug)]
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

fn flip(map: &mut Map, dirs: &[Direction]) {
    let mut x = 0;
    let mut y = 0;
    for dir in dirs {
        use Direction::*;
        //println!("at {}, {}", x, y);
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
        //println!("moved {:?} to {}, {}", dir, x, y);
    }
    let coord = (x, y);
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

fn parse_flip_count(s: &str) -> usize {
    let all_dirs = parse_input(s);
    let mut map = Map::new();
    for dirs in all_dirs {
        flip(&mut map, &dirs);
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
wseweeenwnesenwwwswnew"), 10);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", parse_flip_count(&input));
}
