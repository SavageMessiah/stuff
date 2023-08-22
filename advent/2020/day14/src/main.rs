use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MASK: Regex = Regex::new(r"mask = ([01X]{36})").unwrap();
    static ref SET: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

struct State {
    and_mask: u64,
    or_mask: u64,
    mem: HashMap<usize, u64>
}

impl State {
    fn new() -> State {
        State {
            and_mask: 0,
            or_mask: 0,
            mem: HashMap::new()
        }
    }

    fn run(&mut self, prog: &[Instr]) {
        for instr in prog {
            println!("instr: {:?}, and: {:#038b} or: {:#038b}", instr, self.and_mask, self.or_mask);
            match instr {
                Instr::Mask(bits) => {
                    self.and_mask = bits.iter().map(|b| b.unwrap_or(true) as u64).fold(0, |acc, b| (acc << 1) | b);
                    self.or_mask = bits.iter().map(|b| b.unwrap_or(false) as u64).fold(0, |acc, b| (acc << 1) | b);
                },
                Instr::Mem(loc, val) => {
                    println!("{:#038b} {}", val, val);
                    let val = (val & self.and_mask) | self.or_mask;
                    println!("{:#038b} {}", val, val);
                    self.mem.insert(*loc, val);
                }
            }
        }

    }

    fn sum(&self) -> u64 {
        self.mem.values().sum()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instr {
    Mask([Option<bool>; 36]),
    Mem(usize, u64)
}

impl FromStr for Instr {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Instr::*;
        match (MASK.captures(s), SET.captures(s)) {
            (Some(caps), _) => {
                let mut mask = [None; 36];
                for (i, c) in caps[1].chars().enumerate() {
                    mask[i] = match c {
                        '0' => Some(false),
                        '1' => Some(true),
                        'X' => None,
                        _ => unreachable!("wat")
                    }
                }
                Ok(Mask(mask))
            },
            (_, Some(caps)) => {
                Ok(Mem(caps[1].parse::<usize>()?, caps[2].parse::<u64>()?))
            },
            _ => Err(anyhow!("bad instr {}", s))
        }
    }
}

fn parse_prog(s: &str) -> Result<Vec<Instr>> {
    s.lines().map(|l| l.parse()).collect()
}

#[test]
fn test_run() -> Result<()> {
    let mut state = State::new();
    let instrs = parse_prog("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0")?;
    state.run(&instrs);
    assert_eq!(state.sum(), 165);
    Ok(())
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let instrs = parse_prog(&input)?;
    let mut state = State::new();
    state.run(&instrs);
    let answer = state.sum();
    println!("answer: {:?}", answer);

    Ok(())
}
