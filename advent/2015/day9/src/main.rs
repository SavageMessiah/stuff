use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn distance<'a>(distances: &HashMap<String, HashMap<String,u32>>, path: Vec<&'a String>) -> Option<(Vec<&'a String>, u32)> {
    let mut total = 0;
    for pair in path.windows(2) {
        let distance = distances.get(pair[0])?.get(pair[1])?;
        total += distance;
    }

    Some((path, total))
}

fn main() {
    let re = Regex::new(r"(?P<start>[[:alpha:]]+) to (?P<end>[[:alpha:]]+) = (?P<distance>\d+)").unwrap();
    let mut distances = HashMap::new();

    for line in include_str!("input.txt").lines() {
        let caps = re.captures(line).unwrap();
        let start = caps["start"].to_string();
        let end = caps["end"].to_string();
        let distance = caps["distance"].parse::<u32>().unwrap();

        distances.entry(start.clone()).or_insert(HashMap::new()).insert(end.clone(), distance);
        distances.entry(end.clone()).or_insert(HashMap::new()).insert(start.clone(), distance);
    }

    let paths = distances.keys().permutations(distances.len()).
        map(|path| distance(&distances, path)).
        collect::<Option<Vec<_>>>().unwrap();

    let shortest = paths.iter().min_by_key(|(_, dist)| dist).unwrap();
    let longest = paths.iter().max_by_key(|(_, dist)| dist).unwrap();

    println!("{} paths", paths.len());
    println!("shortest: {:?} of distance {}", shortest.0, shortest.1);
    println!("longest: {:?} of distance {}", longest.0, longest.1);
}
