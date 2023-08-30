use std::{fmt::{Debug, Write}, iter::once, str::FromStr, collections::{HashMap, HashSet}};

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
struct PartialSolve<'a> {
    fits: [[Option<&'a Tile>; 12]; 12],
    seen: HashSet<u32>
}

impl<'a> PartialSolve<'a> {
    fn match_spec_at(&self, idx: (usize, usize)) -> [MatchType; 4] {
        let (x, y) = idx;

        [if y == 0 {
             MatchType::None
         } else {
             MatchType::This(self.fits[y - 1][x].unwrap().edges[2])
         },
         if x == 11 {
             MatchType::None
         } else {
             MatchType::Any
         },
         if y == 11 {
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
        for y in 0..12 {
            for x in 0..12 {
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

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let tiles = parse_tiles(&input);
    let all = permute_tiles(&tiles);
    let index = index(&all);

    let partial = PartialSolve {
        fits: [[None; 12]; 12],
        seen: HashSet::new()
    };

    let sol = partial.solve(&index).unwrap();

    for row in sol {
        for col in row {
            print!("{} ", col.id);
        }
        println!("");
    }

}
