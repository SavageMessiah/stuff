use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::str::FromStr;

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32)
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, amount) = s.split_once(" ").ok_or(anyhow!("bad command: {}", s))?;
        let amount = amount.parse()?;
        use Command::*;
        match command {
            "forward" => Ok(Forward(amount)),
            "down" => Ok(Down(amount)),
            "up" => Ok(Up(amount)),
            _ => Err(anyhow!("unknown command: {}", command))
        }
    }
}

struct Sub {
    pos: u32,
    depth: u32
}

impl Sub {
    fn exec(&mut self, commands: &[Command]) {
        for c in commands {
            use Command::*;
            match c {
                Forward(a) => self.pos += a,
                Down(a) => self.depth += a,
                Up(a) => self.depth -= a,
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let commands = input.lines()
                        .map(|l| l.parse())
        .collect::<Result<Vec<Command>, _>>()?;

    let mut sub = Sub {
        pos: 0,
        depth: 0,
    };

    sub.exec(&commands);

    let answer = sub.pos * sub.depth;

    println!("answer {}", answer);

    Ok(())
}
