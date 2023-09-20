use std::{collections::{HashMap, HashSet}, str::FromStr};

use anyhow::anyhow;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Cave {
    Start,
    Small(String),
    Big(String),
    End
}

impl FromStr for Cave {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            cave => if cave.chars().all(|c| c.is_ascii_uppercase()) {
                Ok(Cave::Big(cave.to_string()))
            } else {
                Ok(Cave::Small(cave.to_string()))
            }
        }
    }
}

type Caves = HashMap<Cave, HashSet<Cave>>;
type Path = Vec<Cave>;

fn has_double_small_cave(path: &Path) -> bool {
    let mut seen = HashSet::new();
    for cave in path {
        match cave {
            cave @ Cave::Small(_) => if seen.contains(&cave) {
                return true;
            } else {
                seen.insert(cave);
            },
            _ => ()
        }
    }

    false
}

fn extend_paths(caves: &Caves, starts: &Vec<Path>) -> Vec<Path> {
    use Cave::*;
    let mut extended = vec![];
    for path in starts {
        println!("considering path {:?}: ", path);
        let last = path.last().unwrap();
        if *last == End {
            println!("\tpath is already complete");
            extended.push(path.clone());
            continue;
        }

        let Some(next_caves) = caves.get(last) else {
            println!("\tpath at dead end");
            continue;
        };

        for next in next_caves {
            print!("\t({:?}) ", next);
            match next {
                Start => {
                    println!("path looped to start");
                    continue;
                },
                cave @ Small(_) if has_double_small_cave(path) && path.contains(cave) => {
                    println!("small cave already passed twice");
                    continue;
                },
                cave => {
                    println!("adding cave to path");
                    let mut new_path = path.clone();
                    new_path.push(cave.clone());
                    extended.push(new_path);
                }
            }
        }
    }

    extended
}

fn all_paths(caves: &Caves) -> Vec<Path> {
    let mut paths = vec![vec![Cave::Start]];
    while paths.iter().any(|p| p.last().unwrap() != &Cave::End) {
        paths = extend_paths(caves, &paths);
    }

    paths
}

fn parse_input(input: &str) -> anyhow::Result<Caves> {
    let mut caves = Caves::new();
    for l in input.lines() {
        let (from, to) = l.split_once('-').ok_or(anyhow!("bad path rule {}", l))?;
        let from: Cave = from.parse()?;
        let to: Cave = to.parse()?;
        caves.entry(from.clone()).or_insert_with(HashSet::new).insert(to.clone());
        caves.entry(to).or_insert_with(HashSet::new).insert(from);
    }
    Ok(caves)
}

#[test]
fn test_parse_and_answer() {
    let caves = parse_input("start-A
start-b
A-c
A-b
b-d
A-end
b-end").unwrap();

    println!("{:?}", caves);

    let paths = all_paths(&caves);
    for path in &paths {
        println!("{:?}", path);
    }

    assert_eq!(paths.len(), 36);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let caves = parse_input(&input)?;
    let paths = all_paths(&caves);

    println!("{}", paths.len());

    Ok(())
}
