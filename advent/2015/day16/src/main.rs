use anyhow::{anyhow, Result};
use lazy_regex::regex;
use std::collections::HashMap;
use std::iter::FromIterator;

fn parse_line(s: &str) -> Result<HashMap<String, i32>> {
    let mut props = HashMap::new();
    for caps in regex!(r"(?P<prop>\w+): (?P<amt>\d+)").captures_iter(s) {
        props.insert(caps["prop"].to_string(), caps["amt"].parse()?);
    }

    if props.is_empty() {
        return Err(anyhow!("no props"))
    }

    Ok(props)
}

fn prop_check(target: &HashMap<String, i32>, prop: &str, amt: i32) -> bool {
    let val = target[prop];
    match prop {
        "cats" | "trees" => val < amt,
        "pomeranians" | "goldfish" => val > amt,
        _ => val == amt
    }
}

fn main() -> Result<()> {
    let aunts = include_str!("input.txt").lines().map(parse_line).collect::<Result<Vec<HashMap<String, i32>>>>()?;
    let target: HashMap<String, i32> = HashMap::from_iter([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);

    let sol = aunts.iter().enumerate().filter(|(_, aunt)| {
        aunt.iter().all(|(prop, &amt)| {
            prop_check(&target, prop, amt)
        })
    });

    for (n, aunt) in sol {
        println!("Aunt {}: {:?}", n + 1, aunt);
    }

    Ok(())
}
