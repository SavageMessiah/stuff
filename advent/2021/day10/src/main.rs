#[derive(Debug, Eq, PartialEq)]
enum Status {
    Ok,
    Incomplete,
    Corrupt(char),
    BadChar(char)
}

impl Status {
    fn score(&self) -> u64 {
        use Status::*;
        match self {
            Corrupt(c) => match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!()
            },
            _ => 0
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
        Status::Incomplete
    }
}

fn score_input(input: &str) -> u64 {
    input.lines().map(|l| check_line(l).score() ).sum()
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

    assert_eq!(score_input(input), 26397);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    println!("{}", score_input(&input));

    Ok(())
}
