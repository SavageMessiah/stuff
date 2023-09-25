use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use itertools::{Itertools, MinMaxResult};

type Pair = (char, char);

#[derive(Debug)]
struct Polymer {
    pairs: HashMap<Pair, usize>,
    counts: HashMap<char, usize>
}

impl FromStr for Polymer {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s.chars().tuple_windows().counts();
        let counts = s.chars().counts();

        Ok(Polymer {pairs, counts})
    }
}

impl Polymer {
    fn insert_pair(&mut self, pair: Pair, n: usize) {
        *self.pairs.entry(pair).or_insert(0) += n
    }
    fn insert_split(&mut self, pair: Pair, n: usize, insert: char) {
        self.insert_pair((pair.0, insert), n);
        *self.counts.entry(insert).or_insert(0) += n;
        self.insert_pair((insert, pair.1), n);
    }

    fn score(&self) -> usize {
        match self.counts.values().minmax() {
            MinMaxResult::MinMax(min, max) => max - min,
            _ => 0,
        }
    }
}

type Rule = (Pair, char);

fn parse_input(input: &str) -> anyhow::Result<(Polymer, Vec<Rule>)> {
    let (template, rules) = input.split_once("\n\n").ok_or(anyhow!("missing rule section"))?;
    let rules = rules.lines().map(|l| {
        let (pair, insert) = l.split_once(" -> ").ok_or(anyhow!("bad rule {}", l))?;
        Ok((pair.chars().tuple_windows().nth(0).unwrap(), insert.chars().nth(0).ok_or(anyhow!("missing insert char"))?))
    }).collect::<anyhow::Result<Vec<_>>>()?;
    Ok((template.parse()?, rules))
}


fn apply(poly: &Polymer, rules: &[Rule]) -> Polymer {
    let mut new = Polymer {
        pairs: HashMap::new(),
        counts: poly.counts.clone()
    };

    for (pair, count) in &poly.pairs {
        let rule = rules.iter().find(|(rule, _)| rule == pair);
        match rule {
            Some((_, insert)) => new.insert_split(*pair, *count, *insert),
            None => new.insert_pair(*pair, *count)
        }
    }
    new
}

fn apply_n(mut poly: Polymer, rules: &[Rule], n: usize) -> Polymer {
    for i in 1..=n {
        println!("applying rules step {}", i);
        poly = apply(&poly, rules);
    }
    poly
}

#[test]
fn test_parse_apply_score() {
    let (mut polymer, rules) = parse_input("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C").unwrap();

    println!("{:?}", polymer);

    println!("{:?}", apply(&polymer, &rules));

    polymer = apply_n(polymer, &rules, 10);

    assert_eq!(polymer.score(), 1588);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (mut polymer, rules) = parse_input(&input)?;

    polymer = apply_n(polymer, &rules, 40);

    println!("{}", polymer.score());

    Ok(())
}
