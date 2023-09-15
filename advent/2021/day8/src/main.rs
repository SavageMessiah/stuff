use std::collections::HashMap;

#[derive(Debug)]
struct Note {
    signals: Vec<String>,
    digits: Vec<String>
}

impl Note {
    fn identify_digits(&self) -> Vec<Option<u8>> {
        self.digits.iter().map(|d| {
            match d.len() {
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                7 => Some(8),
                _ => None,
            }
        }).collect()
    }
}

fn to_sorted(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    chars.sort();
    chars.iter().collect()
}

fn parse_input(s: &str) -> Vec<Note> {
    s.lines().map(|l| {
        let (signals, digits) = l.split_once(" | ").unwrap();
        Note {
            signals: signals.split(' ').map(to_sorted).collect(),
            digits: digits.split(' ').map(to_sorted).collect()
        }

    }).collect()
}

fn count_identified_digits(notes: &[Note]) -> HashMap<u8, usize> {
    notes.iter().flat_map(Note::identify_digits).flatten().counts()
}

#[test]
fn test_parse_and_count() {
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
    let counts = count_identified_digits(&notes);

    assert_eq!(counts.values().sum::<usize>(), 26);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let notes = parse_input(&input);
    let counts = count_identified_digits(&notes);

    println!("{}", counts.values().sum::<usize>());

    Ok(())
}
