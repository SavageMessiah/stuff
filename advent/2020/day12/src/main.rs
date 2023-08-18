use std::{str::FromStr};

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
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn to_polar(&self) -> (f64, f64) {
        let fx = self.x as f64;
        let fy = self.y as f64;
        (fx.hypot(fy), fy.atan2(fx))
    }

    fn rotate(&self, deg: i64) -> Point {
        let (r, rads) = self.to_polar();
        let rads = (rads.to_degrees() + deg as f64).to_radians();
        let (sin, cos) = rads.sin_cos();
        Point {
            x: (r * cos).round() as i64,
            y: (r * sin).round() as i64
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Ship {
    pos: Point,
    waypoint: Point,
}

impl Ship {
    fn go(&mut self, instr: Instr) {
        match instr.op {
            Op::N => self.waypoint.y += instr.val,
            Op::S => self.waypoint.y -= instr.val,
            Op::E => self.waypoint.x += instr.val,
            Op::W => self.waypoint.x -= instr.val,
            Op::L => self.waypoint = self.waypoint.rotate(instr.val),
            Op::R => self.waypoint = self.waypoint.rotate(-instr.val),
            Op::F => {
                self.pos.x += self.waypoint.x * instr.val;
                self.pos.y += self.waypoint.y * instr.val;
            },
        }
    }

    fn manhattan_distance(&self) -> i64 {
        self.pos.x.abs() + self.pos.y.abs()
    }
}

#[test]
fn test_move() {
    let instrs = ["F10", "N3", "F7", "R90", "F11"].iter().map(|i| i.parse()).collect::<Result<Vec<Instr>>>().unwrap();

    let mut ship = Ship {
        pos: Point::default(),
        waypoint: Point { x: 10, y: 1 }
    };
    for instr in instrs {
        println!("ship: {:?} instr: {:?}", ship, instr);
        ship.go(instr);
    }

    assert_eq!(ship.manhattan_distance(), 286);

}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let instrs = input.lines().map(|l| l.parse()).collect::<Result<Vec<Instr>>>()?;
    let mut ship = Ship {
        pos: Point::default(),
        waypoint: Point { x: 10, y: 1 }
    };
    for instr in instrs {
        println!("ship: {:?} instr: {:?}", ship, instr);
        ship.go(instr);
    }

    let answer = ship.manhattan_distance();

    println!("answer: {:?}", answer);

    Ok(())
}
