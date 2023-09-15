use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
struct Note {
    signals: Vec<HashSet<char>>,
    digits: Vec<HashSet<char>>
}

impl Note {
    fn signal_mapping(&self) -> HashMap<char, char> {
        let by_len = self.signals.iter().into_group_map_by(|s| s.len());
        let one = by_len[&2].first().unwrap();
        let seven = by_len[&3].first().unwrap();
        let four = by_len[&4].first().unwrap();
        let four_seven = four.union(seven).copied().collect();

        let mut mapping = HashMap::new();

        let a = seven.difference(&one).nth(0).unwrap();
        mapping.insert(*a, 'a');

        let bd = four.difference(&one).copied().collect::<HashSet<char>>();

        let (five, two_three): (Vec<&HashSet<_>>, Vec<&HashSet<_>>) = by_len[&5].iter().partition(|s| s.intersection(&bd).count() == 2);
        let five = five.first().unwrap();
        let g = five.difference(&four_seven).nth(0).unwrap();
        mapping.insert(*g, 'g');

        for seg in bd {
            if two_three.iter().all(|s| s.contains(&seg)) {
                mapping.insert(seg, 'd');
            } else {
                mapping.insert(seg, 'b');
            }
        }

        for signal in two_three {
            //this signal is 2
            let Some(e) = signal.iter().find(|seg| !mapping.contains_key(seg) && !one.contains(seg)) else { continue; };
            mapping.insert(*e, 'e');
            for seg in one.iter() {
                if signal.contains(seg) {
                    mapping.insert(*seg, 'c');
                } else {
                    mapping.insert(*seg, 'f');
                }
            }

        }

        mapping
    }

    fn as_int(&self) -> u32 {
        let digits = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];
        let mapping = self.signal_mapping();
        self.digits.iter().map(|digit| {
            let mapped = digit.iter().map(|seg| mapping[seg]).sorted().collect::<String>();
            digits.iter().position(|d| *d == mapped).unwrap().to_string()
        }).collect::<String>().parse().unwrap()
    }
}

fn to_signal(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn parse_input(s: &str) -> Vec<Note> {
    s.lines().map(|l| {
        let (signals, digits) = l.split_once(" | ").unwrap();
        Note {
            signals: signals.split(' ').map(to_signal).collect(),
            digits: digits.split(' ').map(to_signal).collect()
        }

    }).collect()
}

#[test]
fn test_parse_and_convert() {
    let notes = parse_input("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
    let ints = notes.iter().map(Note::as_int).collect::<Vec<_>>();

    assert_eq!(ints, vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315]);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let notes = parse_input(&input);
    let answer = notes.iter().map(Note::as_int).sum::<u32>();

    println!("{}", answer);

    Ok(())
}
