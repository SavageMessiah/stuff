use std::collections::HashMap;
use anyhow::{Result, anyhow};
use lazy_static::lazy_static;
use regex::Regex;

type Rules<'a> = HashMap<&'a str, HashMap<&'a str, usize>>;

lazy_static! {
    static ref COUNT: Regex = Regex::new(r"(\d+) (\w+ \w+)").expect("bad regex");
}

fn parse_rule<'a>(s: &'a str) -> Result<(&'a str, HashMap<&'a str, usize>)> {
    let (name, contents) = s.split_once(" bags contain ").ok_or(anyhow!("wat {}", s))?;

    if contents == "no other bags." {
        return Ok((name, HashMap::new()))
    }

    let mut content = HashMap::new();

    for s in contents.split(", ") {
        let caps = COUNT.captures(s).ok_or(anyhow!("wat {}", s))?;
        let n = caps.get(1).unwrap().as_str().parse::<usize>()?;
        content.insert(caps.get(2).unwrap().as_str(), n);
    }


    Ok((name, content))
}

fn bag_count(start: &str, rules: &Rules) -> usize {
    let contents = rules.get(start).unwrap(); //assuming references are all valid
    println!("bag: {} contents: {:?}", start, contents);

    let direct: usize = contents.values().sum();

    let total = direct + contents.iter().map(|(b, c)| bag_count(b, rules) * c).sum::<usize>();
    println!("bag: {} direct: {} total: {}", start, direct, total);
    total
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let rules = input.lines().map(parse_rule).collect::<Result<Rules>>()?;
    let answer = bag_count("shiny gold", &rules);

    println!("answer {}", answer);

    Ok(())
}
