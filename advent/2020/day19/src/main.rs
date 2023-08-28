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

fn is_match<'a>(rules: &HashMap<Id, RuleExp>, rule: &RuleExp, i: &'a str) -> Vec<&'a str> {
    println!("matching \"{}\" against {:?}", i, rule);
    use RuleExp::*;
    let res = match rule {
        Text(t) => {
            if i.len() < t.len() {
                vec![]
            } else {
                let (s, rest) = i.split_at(t.len());
                if s == t {
                    vec![rest]
                } else {
                    vec![]
                }
            }
        },
        Seq(ids) => {
            let mut results = vec![i];
            for id in ids {
                let rule = rules.get(id).unwrap();
                results = results.iter().flat_map(|i| is_match(rules, rule, i)).collect();
                if results.is_empty() {
                    break;
                }
            }
            results
        },
        Or(l, r) => {
            let mut res = vec![];
            res.extend(is_match(rules, l, i));
            res.extend(is_match(rules, r, i));
            res
        }
    };
    println!("matched \"{}\" against {:?} result {:?}", i, rule, res);
    res
}

fn is_match_root(rules: &HashMap<Id, RuleExp>, i: &str) -> bool {
    let root = rules.get(&0).unwrap();
    is_match(rules, root, i).contains(&"")
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

#[test]
fn test_loop() {
    let (rules, messages) = parse_input(
"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1
8: 42 | 42 8
11: 42 31 | 42 11 31

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"
);
    let valid = messages.iter().filter(|m| is_match_root(&rules, m)).collect::<Vec<_>>();
    for m in &valid {
        println!("valid: {}", m);
    }
    assert_eq!(valid.len(), 12);
}

#[test]
fn test_loop_simple() {
    let (rules, messages) = parse_input(
"0: 3 4
1: \"a\"
2: \"b\"
3: 1 | 1 3
4: 1 2 | 1 4 2

aaab
aab
aaabb
aaaaabb
"
);
    let valid = messages.iter().filter(|m| is_match_root(&rules, m)).collect::<Vec<_>>();
    for m in &valid {
        println!("valid: {}", m);
    }
    assert_eq!(valid.len(), 4);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (mut rules, messages) = parse_input(&input);
    rules.insert(8, or.parse_peek("42 | 42 8").unwrap().1);
    rules.insert(11, or.parse_peek("42 31 | 42 11 31").unwrap().1);

    let answer = messages.iter().filter(|m| is_match_root(&rules, m)).count();
    println!("answer: {:?}", answer);
}
