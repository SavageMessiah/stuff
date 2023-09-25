use std::collections::HashSet;

use anyhow::anyhow;
use itertools::{Itertools, MinMaxResult};

type Polymer = String;
type Rule = ((char, char), char);

fn parse_input(input: &str) -> anyhow::Result<(Polymer, Vec<Rule>)> {
    let (template, rules) = input.split_once("\n\n").ok_or(anyhow!("missing rule section"))?;
    let rules = rules.lines().map(|l| {
        let (pair, insert) = l.split_once(" -> ").ok_or(anyhow!("bad rule {}", l))?;
        Ok((pair.chars().tuple_windows().nth(0).unwrap(), insert.chars().nth(0).ok_or(anyhow!("missing insert char"))?))
    }).collect::<anyhow::Result<Vec<_>>>()?;
    Ok((template.to_string(), rules))
}

fn apply(poly: &Polymer, rules: &[Rule]) -> Polymer {
    let mut new = poly[0..1].to_string();
    for pair in poly.chars().tuple_windows::<(_, _)>() {
        for (rule, insert) in rules {
            if *rule == pair {
                new.push(*insert);
                break;
            }
        }
        new.push(pair.1);
    }
    new
}

fn apply_n(poly: &Polymer, rules: &[Rule], n: usize) -> Polymer {
    let mut poly = poly.clone();
    for i in 1..=n {
        println!("applying rules step {}", i);
        poly = apply(&poly, rules);
    }
    poly
}

fn score(poly: &Polymer) -> usize {
    match poly.chars().counts().values().minmax() {
        MinMaxResult::MinMax(min, max) => max - min,
        _ => 0,
    }
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

    assert_eq!(apply(&polymer, &rules), "NCNBCHB");

    polymer = apply_n(&polymer, &rules, 10);

    assert_eq!(score(&polymer), 1588);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (mut polymer, rules) = parse_input(&input)?;

    polymer = apply_n(&polymer, &rules, 10);

    println!("{}", score(&polymer));

    Ok(())
}
