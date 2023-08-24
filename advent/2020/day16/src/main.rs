use std::{collections::HashMap, str::FromStr, ops::RangeInclusive};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RULE: Regex = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

type FieldVal = u64;
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
    fn valid_tickets(&self) -> Vec<&Ticket> {
        self.nearby.iter().filter(move |t| t.iter().all(|n| self.matches_any_rule(*n))).collect()
    }
    fn fields(&self) -> Vec<String> {
        let valid = self.valid_tickets();
        let mut fields: Vec<Option<String>> = vec![None; self.mine.len()];
        while fields.iter().any(|f| f.is_none()) {
            for (name, rule) in &self.rules {
                let matching_cols = (0..self.mine.len()).filter(|i| {
                    valid.iter().all(|t| matches_rule(t[*i], &rule) && fields[*i].is_none())
                }).collect::<Vec<_>>();
                if matching_cols.len() == 1 {
                    println!("found unique match for {} at {}", name, matching_cols[0]);
                    fields[matching_cols[0]] = Some(name.clone());
                }
            }
        }

        fields.into_iter().map(|f| f.unwrap()).collect() //every field should be mapped
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
    let fields = parsed.fields();
    let answer: FieldVal = fields.iter().zip(parsed.mine.iter()).filter_map(|(f, v)| {
        if f.starts_with("departure") {
            println!("{} {}", f, v);
            Some(v)
        } else {
            None
        }
    }).product();

    println!("answer: {:?}", answer);

    Ok(())
}
