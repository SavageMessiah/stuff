use std::{fmt::{Debug, Write}, iter::once, str::FromStr, collections::{HashMap, HashSet}, default};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Tile {
    id: u32,
    grid: Grid<10, bool>,
    edges: [Edge; 4],
}

impl Tile {
    fn new(id: u32, grid: Grid<10, bool>) -> Self {
        let edges = grid.edges();
        Tile {
            id, grid, edges
        }
    }

    fn permutations(&self) -> impl Iterator<Item = Tile> {
        let id = self.id;
        self.grid.permutations().map(move |g| Tile::new(id, g))
    }
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    let mut tiles = vec![];
    for t in input.split("\n\n") {
        if t.is_empty() {
            continue;
        }
        let (id, grid) = t.split_once('\n').unwrap();

        tiles.push(Tile::new(
            id.split(' ').nth(1).unwrap().strip_suffix(":").unwrap().parse().unwrap(),
            grid.parse().unwrap(),
        ))
    }
    tiles
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Grid<const DIM: usize, T: Copy + Default>([[T; DIM]; DIM]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Transform {
    L, R, H, V
}

impl<const DIM: usize, T: Copy + Default> Grid<DIM, T> {
    fn trans(&self, t: Transform) -> Grid<DIM, T> {
        let mut new = [[T::default(); DIM]; DIM];
        let Grid(old) = self;
        use Transform::*;
        match t {
            H => {
                for i in 0..DIM {
                    new[i].copy_from_slice(&old[i]);
                    new[i].reverse();
                }
            },
            V => {
                for i in 0..DIM {
                    new[i] = old[DIM - 1 - i];
                }
            },
            R => {
                for i in 0..DIM {
                    for j in 0..DIM {
                        new[i][j] = old[DIM - 1 - j][i];
                    }
                }
            },
            L => {
                for i in 0..DIM {
                    for j in 0..DIM {
                        new[i][j] = old[j][DIM - 1 - i];
                    }
                }
            }
        }
        Grid(new)
    }

    fn permutations(&self) -> impl Iterator<Item = Self> {
        use Transform::*;
        let perms = [vec![L], vec![R], vec![H], vec![V], vec![L, L], vec![L, V], vec![L, H]]
            .map(|ts| ts.iter().fold(self.clone(), |g, t| g.trans(*t)));
        once(self.clone()).chain(perms)
    }

    fn edges(&self) -> [[T; DIM]; 4] {
        let top = self.0[0];
        let bottom = self.0[DIM - 1];
        let mut left = [T::default(); DIM];
        let mut right = [T::default(); DIM];

        for i in 0..DIM {
            left[i] = self.0[i][0];
            right[i] = self.0[i][DIM - 1];
        }

        [top, right, bottom, left]
    }
}

impl<const DIM: usize> Debug for Grid<DIM, bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            for col in row {
                f.write_char(if col {
                    '#'
                } else {
                    '.'
                })?;
            }
            f.write_char('\n')?;
        }
        std::fmt::Result::Ok(())
    }

}

impl FromStr for Grid<10, bool> {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut g = [[false; 10]; 10];
        let mut y= 0;
        for l in s.lines() {
            let mut x = 0;
            for c in l.chars() {
                if c == '#' {
                    g[y][x] = true;
                }
                x += 1;
            }
            y += 1;
        }
        Ok(Grid(g))
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
enum MatchType {
    None,
    This(Edge),
    Any
}

type Edge = [bool; 10];
type Index<'a> = HashMap<(MatchType, usize), HashSet<&'a Tile>>;

fn permute_tiles(tiles: &[Tile]) -> Vec<Tile> {
    tiles.iter().flat_map(|t| t.permutations()).collect()
}

fn index(tiles: &[Tile]) -> Index {
    let mut edge_to_id = HashMap::new();
    for t in tiles {
        for e in t.edges {
            edge_to_id.entry(e).or_insert(HashSet::new()).insert(t.id);
        }
    }
    let mut index = HashMap::new();
    for t in tiles {
        for i in 0..4 {
            let edge = t.edges[i];
            let alone = edge_to_id.get(&edge).unwrap().len() == 1;
            let key = (if alone {
                MatchType::None
            } else {
                MatchType::This(edge)
            },
                       i);
            index.entry(key).or_insert(HashSet::new()).insert(t);
        }
    }
    index
}

fn find_matching_tiles<'a>(m: [MatchType; 4], seen: &HashSet<u32>, index: &'a Index) -> Vec<&'a Tile> {
    let mut candidates: Option<HashSet<&Tile>> = None;
    for (i, mt) in m.iter().enumerate() {
        if *mt == MatchType::Any {
            continue;
        }
        let key = (*mt, i);
        match (candidates, index.get(&key)) {
            (_, None) => {
                println!("no hit at all for {:?}", key);
                candidates = None;
                break;
            },
            (None, Some(cs)) => {
                println!("first hit for {:?}, {} matches", key, cs.len());
                candidates = Some(cs.clone())
            },
            (Some(current), Some(more)) => {
                let matches = current.intersection(more).copied().collect::<HashSet<_>>();
                println!("found more hits for {:?}, {} matches, {} after intersection", key, more.len(), matches.len());
                candidates = Some(matches);
            }
        }
    }

    let mut matches = vec![];
    match candidates {
        None => (),
        Some(candidates) => {
            for c in candidates {
                if !seen.contains(&c.id) {
                    matches.push(c);
                }
            }
        }
    }
    matches
}

#[derive(Clone, Debug)]
struct PartialSolve<'a, const DIM: usize> {
    fits: [[Option<&'a Tile>; DIM]; DIM],
    seen: HashSet<u32>
}

impl<'a, const DIM: usize> PartialSolve<'a, DIM> {
    fn match_spec_at(&self, idx: (usize, usize)) -> [MatchType; 4] {
        let (x, y) = idx;

        [if y == 0 {
             MatchType::None
         } else {
             MatchType::This(self.fits[y - 1][x].unwrap().edges[2])
         },
         if x == DIM - 1 {
             MatchType::None
         } else {
             MatchType::Any
         },
         if y == DIM - 1 {
             MatchType::None
         } else {
             MatchType::Any
         },
         if x == 0 {
             MatchType::None
         } else {
             MatchType::This(self.fits[y][x - 1].unwrap().edges[1])
         }]
    }

    fn solve(&self, index: &'a Index) -> Option<Vec<Vec<&'a Tile>>> {
        for y in 0..DIM {
            for x in 0..DIM {
                if self.fits[y][x].is_some() {
                    continue;
                }

                let idx = (x, y);
                let ms = self.match_spec_at(idx);
                println!("Checking for candidates at {:?}: {:?}", idx, ms);
                let candidates = find_matching_tiles(ms, &self.seen, index);
                println!("Found {} candidates", candidates.len());
                if candidates.is_empty() {
                    return None;
                }

                for tile in candidates {
                    println!("trying candidate {} \n{:?}", tile.id, tile.grid);
                    let mut next = self.clone();
                    next.fits[y][x] = Some(tile);
                    next.seen.insert(tile.id);
                    if let done @ Some(_) = next.solve(index) {
                        return done;
                    }
                }
                return None;
            }
        }

        let mut done = vec![];
        for row in self.fits {
            let mut done_row = vec![];
            for col in row {
                done_row.push(col.unwrap());
            }
            done.push(done_row);
        }
        Some(done)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Pixel {
    #[default]
    Empty,
    Wave,
    Monster
}

impl<const DIM: usize> Debug for Grid<DIM, Pixel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            for col in row {
                use Pixel::*;
                f.write_char(match col {
                    Empty => '.',
                    Wave => '#',
                    Monster => 'O'
                })?;
            }
            f.write_char('\n')?;
        }
        std::fmt::Result::Ok(())
    }
}

fn assemble_image<const DIM: usize>(sol: &Vec<Vec<&Tile>>) -> Grid<DIM, Pixel> {
    let mut g = [[Pixel::Empty; DIM]; DIM];
    for y in 0..DIM {
        for x in 0..DIM {
            let tile = sol[y / 8][x / 8];
            g[y][x] = if tile.grid.0[1 + y % 8][1 + x % 8] {
                Pixel::Wave
            } else {
                Pixel::Empty
            };
        }
    }
    Grid(g)
}

fn parse_pattern(s: &str) -> Vec<Vec<bool>> {
    s.lines().map(|l| {
        l.chars().map(|c| c == '#').collect()
    }).collect()
}

fn print_pattern(p: &Vec<Vec<bool>>) {
    for r in p {
        for c in r {
            print!("{}", if *c { '#' } else { ' ' });
        }
        println!("");
    }
}

fn match_pattern<const DIM: usize>(image: &mut Grid<DIM, Pixel>, at: (usize, usize), pattern: &Vec<Vec<bool>>) -> bool {
    let needed = pattern.iter().map(|row| row.iter().filter(|p| **p).count()).sum();
    let (x, mut y) = at;
    let mut matched = 0usize;
    for row in pattern {
        let mut x = x;
        for pixel in row {
            if image.0[y][x] == Pixel::Wave && *pixel {
                matched += 1;
            }
            x += 1;
        }
        y += 1;
    }
    matched == needed
}

fn mark_pattern<const DIM: usize>(image: &mut Grid<DIM, Pixel>, at: (usize, usize), pattern: &Vec<Vec<bool>>) {
    for y in 0..pattern.len() {
        for x in 0..pattern[0].len() {
            if pattern[y][x] {
                image.0[at.1 + y][at.0 + x] = Pixel::Monster;
            }
        }
    }
}

fn find_all_pattern_matches<const DIM: usize>(image: &mut Grid<DIM, Pixel>, pattern: &Vec<Vec<bool>>) -> usize {
    let pattern_width = pattern[0].len();
    let pattern_height = pattern.len();
    let mut found = 0;
    for y in 0..(DIM - pattern_height) {
        for x in 0..(DIM - pattern_width) {
            if match_pattern(image, (x, y), pattern) {
                mark_pattern(image, (x, y), pattern);
                found += 1;
            }
        }
    }

    found
}

fn find_and_mark<const DIM: usize>(image: &Grid<DIM, Pixel>, pattern: &Vec<Vec<bool>>) -> Grid<DIM, Pixel> {
    for mut p in image.permutations() {
        if find_all_pattern_matches(&mut p, pattern) > 0 {
            return p
        }
    }
    unreachable!("shit");
}

fn count_waves<const DIM: usize>(image: &Grid<DIM, Pixel>) -> usize {
    image.0.iter().map(|row| row.iter().filter(|p| **p == Pixel::Wave).count()).sum()
}

fn the_whole_thing<const TILE: usize, const PIXEL: usize>(input: &str) -> usize {
    let tiles = parse_tiles(input);
    let all = permute_tiles(&tiles);
    let index = index(&all);

    let partial = PartialSolve {
        fits: [[None; TILE]; TILE],
        seen: HashSet::new()
    };

    let sol = partial.solve(&index).unwrap();

    for row in &sol {
        for col in row {
            print!("{} ", col.id);
        }
        println!("");
    }

    let image = assemble_image::<PIXEL>(&sol);

    println!("{:?}", image);

    let pattern = parse_pattern(
"                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ");

    print_pattern(&pattern);

    let marked = find_and_mark(&image, &pattern);

    println!("{:?}", marked);

    count_waves(&marked)
}

#[test]
fn test() {
    let input =
"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    assert_eq!(the_whole_thing::<3, 24>(input), 273);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let answer = the_whole_thing::<12, 96>(&input);

    println!("Wave pixels: {}", answer);
}
