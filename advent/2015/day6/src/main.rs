use anyhow::{anyhow, Result};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    combinator::{map_res, value},
    sequence::{delimited, separated_pair},
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

fn parse_rule(s: &str) -> IResult<&str, Rule> {
    let (s, change) = parse_change(s)?;
    let (s, from) = delimited(space1, parse_coord, space1)(s)?;
    let (s, _) = tag("through")(s)?;
    let (s, to) = delimited(space1, parse_coord, space0)(s)?;

    Ok((s, Rule{
        change: change,
        from: from,
        to: to,
    }))
}

fn parse_rules() -> Result<Vec<Rule>> {
    let rules = include_str!("input.txt").
        lines().
        map(parse_rule).
        map(|t| t.map(|r| r.1)).
        collect::<Result<Vec<_>, _>>();
    match rules {
        Ok(r) => Ok(r),
        Err(e) => Err(anyhow!(e)),
    }
}

type Grid = [[bool; 1000]; 1000];

fn apply_rule(grid: &mut Grid, rule: &Rule) {
    for y in rule.from.1..=rule.to.1 {
        for x in rule.from.0..=rule.to.0 {
            use Change::*;
            match rule.change {
                On => grid[y][x] = true,
                Off => grid[y][x] = false,
                Toggle => grid[y][x] = !grid[y][x]
            }
        }

    }
}

fn main() -> Result<()> {
    let mut g = [[false; 1000]; 1000];
    let rules = parse_rules()?;
    for r in &rules {
        apply_rule(&mut g, r);
    }

    let mut count = 0;
    for row in g.iter() {
        for col in row.iter() {
            if *col {
                count += 1
            }

        }
    }

    println!("Lit: {}", count);
    Ok(())
}
