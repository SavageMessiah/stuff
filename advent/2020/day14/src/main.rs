use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MASK: Regex = Regex::new(r"mask = ([01X]{36})").unwrap();
    static ref SET: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

struct Mask {
    or: u64,
    floating: Vec<u64>
}

impl Mask {
    fn expand_address(&self, addr: u64) -> Vec<u64> {
        println!("starting addr: {:#038b} {}", addr, addr);
        let mut addrs = vec![addr | self.or];

        for bit in &self.floating {
            println!("setting floating bit: {:#038b}", bit);
            let mut more_addrs = addrs.clone();
            for addr in addrs.iter_mut() {
                *addr |= *bit;
            }
            let bit = (!*bit) & 0xFFFFFFFFF;
            for addr in more_addrs.iter_mut() {
                *addr &= bit;
            }
            addrs.extend(more_addrs);
        }
        for a in &addrs {
            println!("expanded: {:#038b} {}", a, a);
        }
        addrs
    }
}

impl From<&[Option<bool>; 36]> for Mask {
    fn from(source: &[Option<bool>; 36]) -> Self {
        Mask {
            or: source.iter().map(|b| b.unwrap_or(false) as u64).fold(0, |acc, b| (acc << 1) | b),
            floating: source.iter().enumerate().filter_map(|(i, b)| match b {
                Some(_) => None,
                None => Some(1 << (35 - i))
            }).collect()
        }
    }
}

struct State {
    mask: Mask,
    mem: HashMap<u64, u64>
}

impl State {
    fn new() -> State {
        State {
            mask: Mask { or: 0, floating: vec![] },
            mem: HashMap::new()
        }
    }

    fn run(&mut self, prog: &[Instr]) {
        for instr in prog {
            match instr {
                Instr::Mask(mask) => {
                    self.mask = mask.into();
                },
                Instr::Mem(addr, val) => {
                    let addrs = self.mask.expand_address(*addr);
                    for addr in addrs {
                        self.mem.insert(addr, *val);
                    }
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
    Mem(u64, u64)
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
                Ok(Mem(caps[1].parse::<u64>()?, caps[2].parse::<u64>()?))
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
    let instrs = parse_prog("mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1")?;
    state.run(&instrs);
    assert_eq!(state.sum(), 208);
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
