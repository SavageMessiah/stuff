use std::{ops::RangeInclusive, cmp::max};

use anyhow::anyhow;
use itertools::Itertools;

struct Area {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>
}

fn shot(mut vel: (i32, i32), area: &Area) -> Option<i32> {
    let mut pos = (0, 0);
    let mut highest_y = 0;

    print!("shot: {:?} ", vel);
    while (pos.0 < *area.x.start() || pos.1 > *area.y.end()) &&
          !(pos.0 > *area.x.end() || pos.1 < *area.y.start()) {
        pos.0 += vel.0;
        pos.1 += vel.1;

        if pos.1 > highest_y {
            highest_y = pos.1;
        }

        vel.0 = max(0, vel.0 - 1);
        vel.1 = vel.1 - 1;
    }
    print!("final pos: {:?} ", pos);

    if pos.0 > *area.x.end() || pos.1 < *area.y.start() {
        println!("too far");
        None
    } else {
        println!("swish: {}", highest_y);
        Some(highest_y)
    }
}

fn highest_y(area: &Area) -> i32 {
    (1..100).cartesian_product(1..100).filter_map(|vel| shot(vel, area)).max().unwrap()
}

#[test]
fn test_shot() {
    let area = Area {
        x: 20..=30,
        y: -10..=-5
    };
    assert_eq!(shot((6, 9), &area), Some(45));
}

#[test]
fn test_parse_and_highest() {
    let area = parse_input("target area: x=20..30, y=-10..-5").unwrap();
    assert_eq!(highest_y(&area), 45);
}

fn parse_input(input: &str) -> anyhow::Result<Area> {
    let (x, y) = input.trim().split_once(", y=").ok_or(anyhow!("bad area spec {}", input))?;
    let x = x.strip_prefix("target area: x=").ok_or(anyhow!("bad x spec {}", x))?;
    let (sx, ex) = x.split_once("..").ok_or(anyhow!("bad x spec {}", x))?;
    let (sy, ey) = y.split_once("..").ok_or(anyhow!("bad y spec {}", y))?;

    Ok(Area {
        x: sx.parse()?..=ex.parse()?,
        y: sy.parse()?..=ey.parse()?
    })
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let area = parse_input(&input)?;

    println!("{}", highest_y(&area));

    Ok(())
}
