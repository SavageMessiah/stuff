use num::integer::{div_ceil, div_floor, div_mod_floor};
use std::{ops::{RangeInclusive, Range}, collections::{HashSet, HashMap}, cmp::{max, min}};


type Cuboid = [RangeInclusive<i32>; 3];

#[derive(Debug)]
struct Step {
    on: bool,
    cuboid: Cuboid
}

type Coord = [i8; 3];

#[derive(Debug, Eq, PartialEq)]
struct Stride {
    n: i32,
    pow: u32,
    complete: bool
}

fn split(range: Range<i32>, pow: u32) -> Vec<Stride> {
    let pow2 = 2i32.pow(pow);
    let (start_floor, start_mod) = div_mod_floor(range.start, pow2);
    let mut strides = vec![Stride {
            n: start_floor,
            pow,
            complete: start_mod == 0
        }];
    let mut next = start_floor + 1;
    println!("{} {}", next * pow2, range.end);
    while next * pow2 < range.end {
        strides.last_mut().map(|l| l.complete = l.complete && true);
        strides.push(Stride {
            n: next,
            pow,
            complete: true,
        });
        next += 1;
    }

    let last_complete_to_end = range.end == (next * pow2) - 1;
    println!("{} = {} == {}", last_complete_to_end, range.end, (next * pow2) - 1);
    strides.last_mut().map(|l| l.complete = l.complete && last_complete_to_end);

    strides
}

#[test]
fn test_split() {
    assert_eq!(split(-131072..-65537, 16), vec![Stride {
        n: -2,
        pow: 16,
        complete: true,
    }]);

    assert_eq!(split(-131072..-65536, 16), vec![Stride {
        n: -2,
        pow: 16,
        complete: true,
    },
    Stride {
        n: -1,
        pow: 16,
        complete: false
    }]);
}

struct Space {
    bounds: Cuboid,
    pow: usize,
    on: bool,
    children: HashMap<Cuboid, Space>
}

#[derive(Debug)]
enum OctTree {
    Solid {
        origin: [i32; 3],
        size: usize,
    },
    Branch {
        origin: [i32; 3],
        size: usize,
        children: [Option<Box<OctTree>>; 8],
    }
}

fn parse_line(l: &str) -> Step {
    let (state, bounds) = l.split_once(' ').unwrap();
    let on = state == "on";
    let bounds = bounds.split(',').map(|d| {
        let (start, end) = d.split_once("..").unwrap();
        let start = start[2..].parse().unwrap();
        let end = end.parse().unwrap();
        start..=end
    }).collect::<Vec<_>>();

    Step {
        on,
        cuboid: bounds.try_into().unwrap()
    }
}

fn parse_input(s: &str) -> Vec<Step> {
    s.lines().map(parse_line).collect()
}

fn restrict_bound(bound: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    max(*bound.start(), -50)..=min(*bound.end(), 50)
}

fn range_is_overlapping(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

fn is_overlapping(a: &Cuboid, b: &Cuboid) -> bool {
    for i in 0..3 {
        if !range_is_overlapping(&a[i], &b[i]) {
            return false;
        }
    }
    true
}

fn is_contained(a: &Cuboid, b: &Cuboid) -> bool {
    (0..3).all(|i| b[i].contains(a[i].start()) && b[i].contains(a[i].end()))
}

//#[test]
//fn test_parse_and_boot() {
//    let input = "on x=10..12,y=10..12,z=10..12
//on x=11..13,y=11..13,z=11..13
//off x=9..11,y=9..11,z=9..11
//on x=10..10,y=10..10,z=10..10";
//    let steps = parse_input(&input);
//    let on = boot_core(&steps);
//
//    assert_eq!(on.len(), 39);
//}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let steps = parse_input(&input);

    println!("{:?}", split(-80123..3001, 16))

}
