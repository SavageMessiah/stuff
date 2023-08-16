use std::{str::FromStr, collections::HashSet};

use anyhow::{Result, anyhow};

#[derive(Debug)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

struct Instr {
    op: Op,
    arg: i32,
}

impl FromStr for Instr {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (op, arg) = s.split_once(' ').ok_or(anyhow!("bad instr {}", s))?;
        Ok(Instr {
            op: match op {
                "nop" => Op::Nop,
                "acc" => Op::Acc,
                "jmp" => Op::Jmp,
                _ => return Err(anyhow!("bad op {}", op))
            },
            arg: arg.parse()?
        })
    }
}

struct State {
    pc: usize,
    acc: i32,
    run: HashSet<usize>
}

impl State {
    fn run(&mut self, prog: &Vec<Instr>) {
        while !self.run.contains(&self.pc) {
            let instr = &prog[self.pc];
            self.run.insert(self.pc);
            println!("pc: {} acc: {} op: {:?} arg: {}", self.pc, self.acc, instr.op, instr.arg);
            match instr.op {
                Op::Nop => self.pc += 1,
                Op::Acc => {
                    self.acc += instr.arg;
                    self.pc += 1;
                },
                Op::Jmp => self.pc = (self.pc as i32 + instr.arg) as usize,
            }

        }

    }
}


fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let prog = input.lines().map(|l| l.parse::<Instr>()).collect::<Result<Vec<_>>>()?;
    let mut state = State { pc: 0, acc: 0, run: HashSet::new() };
    state.run(&prog);
    let answer = state.acc;

    println!("answer {}", answer);

    Ok(())
}
