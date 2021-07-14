use anyhow::{anyhow, Result};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    combinator::{map_res, value},
    sequence::{delimited, separated_pair, tuple},
};

#[derive(Clone, Debug)]
enum Change {
    On,
    Off,
    Toggle,
}

type Coord = (usize, usize);

#[derive(Clone, Debug)]
struct Rule {
    change: Change,
    from: Coord,
    to: Coord,
}

fn parse_change(s: &str) -> IResult<&str, Change> {
    alt((value(Change::On, tag("turn on")),
         value(Change::Off, tag("turn off")),
         value(Change::Toggle, tag("toggle"))))(s)
}

fn parse_num(s: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(s)
}

fn parse_coord(s: &str) -> IResult<&str, Coord> {
    separated_pair(parse_num, tag(","), parse_num)(s)
}

fn parse_rule(s: &str) -> Result<Rule> {
    let parse = tuple((parse_change,
               delimited(space1, parse_coord, space1),
               tag("through"),
               delimited(space1, parse_coord, space0)))(s);
    match parse {
        Ok((_, (change, from, _, to))) => Ok(Rule{
            change: change,
            from: from,
            to: to,
        }),
        Err(e) => Err(anyhow!("Error in rule '{}': {}", s, e)),
    }
}

fn parse_rules() -> Result<Vec<Rule>> {
    include_str!("input.txt").
        lines().
        map(parse_rule).
        collect::<Result<Vec<_>, _>>()
}

type Grid = [[u32; 1000]; 1000];

fn apply_rule(grid: &mut Grid, rule: &Rule) {
    for y in rule.from.1..=rule.to.1 {
        for x in rule.from.0..=rule.to.0 {
            use Change::*;
            match rule.change {
                On => grid[y][x] += 1,
                Off => {
                    let l = &mut grid[y][x];
                    if *l != 0 {
                        *l -= 1;
                    }
                }
                Toggle => grid[y][x] += 2
            }
        }

    }
}

fn main() -> Result<()> {
    let mut g = [[0; 1000]; 1000];
    let rules = parse_rules()?;
    for r in &rules {
        apply_rule(&mut g, r);
    }

    let mut count: u32 = 0;
    for row in g.iter() {
        for col in row.iter() {
            count += *col
        }
    }

    println!("Brightness: {}", count);
    Ok(())
}
