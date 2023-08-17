use std::{str::FromStr, f64::consts::PI};

use anyhow::{Result, anyhow};


#[derive(Clone, Copy, Debug)]
enum Op {
    N,
    S,
    E,
    W,
    L,
    R,
    F
}

#[derive(Clone, Copy, Debug)]
struct Instr {
    op: Op,
    val: i64
}

impl FromStr for Instr {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Instr {
            op: match s.chars().nth(0) {
                Some('N') => Op::N,
                Some('S') => Op::S,
                Some('E') => Op::E,
                Some('W') => Op::W,
                Some('L') => Op::L,
                Some('R') => Op::R,
                Some('F') => Op::F,
                _ => return Err(anyhow!("bad instr {}", s))
            },
            val: s[1..].parse()?
        })
    }
}


#[derive(Clone, Copy, Debug, Default)]
struct Ship {
    x: i64,
    y: i64,
    dir: i64
}

impl Ship {
    fn go(&mut self, instr: Instr) {
        match instr.op {
            Op::N => self.y -= instr.val,
            Op::S => self.y += instr.val,
            Op::E => self.x += instr.val,
            Op::W => self.x -= instr.val,
            Op::L => {self.dir -= instr.val; self.dir %= 360},
            Op::R => {self.dir += instr.val; self.dir %= 360},
            Op::F => {
                let a = self.dir as f64 * (PI / 180.0) ;
                let d = instr.val as f64;
                let x = (a.cos() * d).round() as i64;
                let y = (a.sin() * d).round() as i64;
                self.x += x;
                self.y += y;
            },
        }
    }

    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let instrs = input.lines().map(|l| l.parse()).collect::<Result<Vec<Instr>>>()?;
    let mut ship = Ship::default();
    for instr in instrs {
        println!("ship: {:?} instr: {:?}", ship, instr);
        ship.go(instr);
    }

    let answer = ship.manhattan_distance();

    println!("answer: {:?}", answer);

    Ok(())
}
