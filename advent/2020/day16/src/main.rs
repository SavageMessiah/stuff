use std::{collections::HashMap, str::FromStr, ops::RangeInclusive};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RULE: Regex = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

type FieldVal = u32;
type Ticket = Vec<FieldVal>;
type Rule = (RangeInclusive<FieldVal>, RangeInclusive<FieldVal>);

struct Input {
    rules: HashMap<String, Rule>,
    mine: Ticket,
    nearby: Vec<Ticket>
}

fn matches_rule(n: FieldVal, rule: &Rule) -> bool {
    rule.0.contains(&n) || rule.1.contains(&n)
}

impl Input {
    fn matches_any_rule(&self, n: FieldVal) -> bool {
        self.rules.values().any(|rule| matches_rule(n, rule))
    }
    fn error_rate(&self) -> FieldVal {
        self.nearby.iter().flat_map(|t| t.iter()).filter(|fv| !self.matches_any_rule(**fv)).sum()
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");

        let mut rules = HashMap::new();
        for r in parts.next().ok_or(anyhow!("missing rules"))?.lines() {
            let caps = RULE.captures(r).ok_or(anyhow!("bad rule {}", r))?;
            rules.insert(caps[1].to_string(), (caps[2].parse()?..=caps[3].parse()?, caps[4].parse()?..=caps[5].parse()?));
        }

        let mine = parts.next()
                        .ok_or(anyhow!("missing my ticket section"))?
                        .lines()
                        .nth(1)
                        .ok_or(anyhow!("missing my ticket"))?
                        .split(',')
                        .map(|n| n.parse())
            .collect::<Result<Ticket, _>>()?;

        let nearby = parts.next()
                          .ok_or(anyhow!("missing nearby tickets section"))?
                          .lines()
                          .skip(1)
                          .map(|l| l.split(',').map(|n| n.parse::<FieldVal>()).collect::<Result<Ticket, _>>())
                          .collect::<Result<Vec<Ticket>, _>>()?;

        Ok(Input {
            rules,
            mine,
            nearby
        })
    }
}


fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let parsed = input.parse::<Input>()?;
    let answer = parsed.error_rate();
    println!("answer: {:?}", answer);

    Ok(())
}
