use std::str::FromStr;

use winnow::prelude::*;
use winnow::{
    ascii::{digit1 as digits, space0 as spaces},
    combinator::alt,
    combinator::delimited,
    combinator::fold_repeat,
    token::one_of,
};

fn expr(i: &mut &str) -> PResult<i64> {
    let init = term.parse_next(i)?;

    fold_repeat(0..,
                (one_of(['+', '*']), term),
                move || init,
    |acc, (op, val): (char, i64)| {
        match op {
            '+' => acc + val,
            '*' => acc * val,
            _ => unreachable!("unhandled op")
        }
    }).parse_next(i)
}

fn term(i: &mut &str) -> PResult<i64> {
    delimited(
        spaces,
        alt((digits.try_map(FromStr::from_str),
            delimited('(', expr, ')'))),
        spaces).parse_next(i)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let answer: i64 = input.lines().map(|l| expr.parse(l)).collect::<Result<Vec<_>, _>>().unwrap().iter().sum();
    println!("answer: {:?}", answer);
}
