use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
enum Status {
    Ok,
    Incomplete(Vec<char>),
    Corrupt(char),
    BadChar(char)
}

impl Status {
    fn score(&self) -> Option<u64> {
        use Status::*;
        match self {
            Incomplete(stack) =>
                Some(stack.iter().rev().fold(0, |score, c|
                                  score * 5 + (match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!()
                }))),
            _ => None
        }
    }
}

fn check_line(line: &str) -> Status {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => if *stack.last().unwrap() == '(' {
                stack.pop();
            } else {
                return Status::Corrupt(c);
            },
            ']' => if *stack.last().unwrap() == '[' {
                stack.pop();
            } else {
                return Status::Corrupt(c);
            },
            '}' => if *stack.last().unwrap() == '{' {
                stack.pop();
            } else {
                return Status::Corrupt(c);
            },
            '>' => if *stack.last().unwrap() == '<' {
                stack.pop();
            } else {
                return Status::Corrupt(c);
            },
            _ => return Status::BadChar(c)
        }
    }
    if stack.is_empty() {
        Status::Ok
    } else {
        Status::Incomplete(stack)
    }
}

fn score_input(input: &str) -> u64 {
    let mut scores = input.lines().filter_map(|l| check_line(l).score() ).collect_vec();
    scores.sort();
    scores[scores.len() / 2]
}

#[test]
fn test_parse_and_answer() {
    let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    assert_eq!(score_input(input), 288957);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    println!("{}", score_input(&input));

    Ok(())
}
