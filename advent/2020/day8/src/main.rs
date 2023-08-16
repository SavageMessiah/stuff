use std::{str::FromStr, collections::HashSet};

use anyhow::{Result, anyhow};

#[derive(Clone, Debug)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone, Debug)]
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
    fn run(&mut self, prog: &Vec<Instr>) -> bool {
        loop {
            if self.run.contains(&self.pc) {
                return false;
            }
            if self.pc == prog.len() {
                return true;
            }
            let instr = &prog[self.pc];
            self.run.insert(self.pc);
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
    for i in 0..prog.len() {
        println!("instr was {:?}", prog[i]);
        let mut prog = prog.clone();
        let instr = prog.get_mut(i).unwrap();
        match instr.op {
            Op::Acc => continue,
            Op::Jmp => instr.op = Op::Nop,
            Op::Nop => instr.op = Op::Jmp
        }
        println!("instr changed to {:?}", prog[i]);
        let mut state = State { pc: 0, acc: 0, run: HashSet::new() };
        if state.run(&prog) {
            println!("terminated after changing instr {} with {}", i, state.acc);
            break;
        } else {
            println!("loop after changing instr {} pc: {}", i, state.pc);
        }
    }

    Ok(())
}
