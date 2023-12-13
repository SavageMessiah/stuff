use std::{ops::RangeInclusive, collections::HashSet, cmp::{max, min}};


#[derive(Debug)]
struct Cuboid {
    on: bool,
    bounds: [RangeInclusive<i32>; 3]
}

fn parse_line(l: &str) -> Cuboid {
    let (state, bounds) = l.split_once(' ').unwrap();
    let on = state == "on";
    let bounds = bounds.split(',').map(|d| {
        let (start, end) = d.split_once("..").unwrap();
        let start = start[2..].parse().unwrap();
        let end = end.parse().unwrap();
        start..=end
    }).collect::<Vec<_>>();

    Cuboid {
        on,
        bounds: bounds.try_into().unwrap()
    }
}

fn parse_input(s: &str) -> Vec<Cuboid> {
    s.lines().map(parse_line).collect()
}

type Coord = [i32; 3];

fn restrict_bound(bound: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    max(*bound.start(), -50)..=min(*bound.end(), 50)
}

fn boot_core(steps: &[Cuboid]) -> HashSet<Coord> {
    let mut on = HashSet::new();
    for step in steps {
        println!("{:?}", step);
        for x in restrict_bound(&step.bounds[0]) {
            for y in restrict_bound(&step.bounds[1]) {
                for z in restrict_bound(&step.bounds[2]) {
                    let coord = [x, y, z];
                    if step.on {
                        on.insert(coord);
                    } else {
                        on.remove(&coord);
                    }
                }
            }
        }
    }
    on
}

#[test]
fn test_parse_and_boot() {
    let input = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
    let steps = parse_input(&input);
    let on = boot_core(&steps);

    assert_eq!(on.len(), 39);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let steps = parse_input(&input);
    let on = boot_core(&steps);

    println!("{}", on.len());
}
