use anyhow::{anyhow, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Rule<'a> {
    target: &'a str,
    amount: i32,
    other: &'a str,
}

fn parse_line<'a>(l: &'a str) -> Result<Rule<'a>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<target>\w+) would (?P<sign>\w+) (?P<amount>\d+) happiness units by sitting next to (?P<other>\w+)\.").unwrap();
    }

    let caps = RE.captures(l).ok_or(anyhow!("no match"))?;
    let sign: i32 = match &caps["sign"] {
        "gain" => Ok(1),
        "lose" => Ok(-1),
        _ => Err(anyhow!("bad sign {}", &caps["sign"]))
    }?;
    let amount = &caps["amount"].parse::<i32>()?;


    Ok(Rule {
        target: caps.name("target").unwrap().as_str(),
        amount: amount * sign,
        other: caps.name("other").unwrap().as_str(),
    })
}

fn happiness<'a>(rules: &HashMap<(&'a str, &'a str), i32>, peeps: &[&'a str]) -> i32 {
    peeps.iter().copied().cycle().tuple_windows::<(&'a str, &'a str)>().take(peeps.len()).map(|p| {
        rules.get(&p).unwrap() + rules.get(&(p.1, p.0)).unwrap()
    }).sum()
}

fn main() -> Result<()> {
    let rules = include_str!("input.txt").lines().map(parse_line).collect::<Result<Vec<_>>>()?;

    let mut people = rules.iter().flat_map(|r| vec![r.target, r.other]).collect::<HashSet<_>>();

    let mut combined = HashMap::new();
    for Rule {target, amount, other} in &rules {
        combined.insert((*target, *other), *amount);
    }

    let me = "me";
    for person in &people {
        combined.insert((me, person), 0);
        combined.insert((person, me), 0);
    }
    people.insert(me);

    let solution = people.iter().copied().permutations(people.len()).map(move |p| {
        let happiness = happiness(&combined, &p);
      (p, happiness)
    }).max_by_key(|(_, h)| *h);

    println!("solution: {:?}", solution);

    Ok(())
}
