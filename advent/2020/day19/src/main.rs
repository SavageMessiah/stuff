use std::collections::HashMap;
use std::str::FromStr;

use winnow::prelude::*;
use winnow::{
    ascii::{alpha1 as alphas, digit1 as digits, space1 as spaces},
    combinator::{alt,
                 delimited,
                 preceded,
                 separated1,
                 separated_pair,
                 terminated},
};

type Id = i32;

#[derive(Debug, Eq, PartialEq)]
enum RuleExp {
    Text(String),
    Seq(Vec<Id>),
    Or(Box<RuleExp>, Box<RuleExp>)
}

fn is_match<'a>(rules: &HashMap<Id, RuleExp>, rule: &RuleExp, i: &'a str) -> Option<&'a str> {
    use RuleExp::*;
    let res = match rule {
        Text(t) => {
            if i.len() < t.len() {
                None
            } else {
                let (s, rest) = i.split_at(t.len());
                if s == t {
                    Some(rest)
                } else {
                    None
                }
            }
        },
        Seq(ids) => {
            let mut i = i;
            for id in ids {
                let rule = rules.get(&id).unwrap();
                if let Some(rest) = is_match(rules, rule, i) {
                    i = rest;
                } else {
                    return None;
                }
            }
            Some(i)
        },
        Or(l, r) => {
            is_match(rules, l, i).or(is_match(rules, r, i))
        }
    };
    println!("matching \"{}\" against {:?} result {:?}", i, rule, res);
    res
}

fn is_match_root(rules: &HashMap<Id, RuleExp>, i: &str) -> bool {
    let root = rules.get(&0).unwrap();
    is_match(rules, root, i) == Some("")
}

fn id(i: &mut &str) -> PResult<Id> {
    digits.try_map(FromStr::from_str).parse_next(i)
}

fn rule(i: &mut &str) -> PResult<(Id, RuleExp)> {
    (terminated(id, ':'), preceded(spaces, exp)).parse_next(i)
}

fn exp(i: &mut &str) -> PResult<RuleExp> {
    alt((text, or, seq)).parse_next(i)
}

fn text(i: &mut &str) -> PResult<RuleExp> {
    delimited('"', alphas, '"').map(|s: &str| RuleExp::Text(s.to_string())).parse_next(i)
}

fn seq(i: &mut &str) -> PResult<RuleExp> {
    separated1(id, spaces).map(RuleExp::Seq).parse_next(i)
}

fn or(i: &mut &str) -> PResult<RuleExp> {
    separated_pair(seq,
                   delimited(spaces, '|', spaces),
                   seq)
        .map(|(l, r)| RuleExp::Or(Box::new(l), Box::new(r)))
        .parse_next(i)
}

fn parse_input(i: &str) -> (HashMap<Id, RuleExp>, Vec<&str>) {
    let (rules, messages) = i.split_once("\n\n").unwrap();

    let rules = rules.lines().map(|l| rule.parse(l).unwrap()).collect();

    (rules, messages.lines().collect())
}

#[test]
fn test_parse() {
    assert_eq!(or.parse_peek("2 3 | 1 2"), Ok(("",
                                               RuleExp::Or(Box::new(RuleExp::Seq(vec![2, 3])),
                                                           Box::new(RuleExp::Seq(vec![1, 2]))))));
    assert_eq!(rule.parse_peek("0: 2 3 | 1 2"), Ok(("",
                                                    (0,
                                                     RuleExp::Or(Box::new(RuleExp::Seq(vec![2, 3])),
                                                                 Box::new(RuleExp::Seq(vec![1, 2])))))));
}

#[test]
fn test_match() {
    let (rules, messages) = parse_input(
"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
");
    let c = messages.iter().filter(|m| is_match_root(&rules, m)).count();
    assert_eq!(c, 2);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (rules, messages) = parse_input(&input);

    let answer = messages.iter().filter(|m| is_match_root(&rules, m)).count();
    println!("answer: {:?}", answer);
}
