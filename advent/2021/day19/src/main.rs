use std::{collections::HashSet, num::ParseIntError};

use anyhow::anyhow;
use itertools::Itertools;

type Beacon = [i32; 3];
type Matrix = [[i32; 3]; 3];
type Scanner = HashSet<Beacon>;

fn transform(beacon: &Beacon, matrix: &Matrix) -> Beacon {
    let mut new = [0; 3];
    for row in 0..3 {
        for col in 0..3 {
            new[row] += beacon[col] * matrix[row][col]
        }
    }
    new
}

#[test]
fn test_transform() {
    let b = [500, -245, 456];
    assert_eq!(transform(&b, &ID), b);
}

static ID: Matrix = [[1, 0, 0],
                     [0, 1, 0],
                     [0, 0, 1]];

static FACING: [Matrix; 6] = [ID,
                              [[0, 1, 0],
                               [-1, 0, 0],
                               [0, 0, 1]],
                              [[-1, 0, 0],
                               [0, -1, 0],
                               [0, 0, 1]],
                              [[0, -1, 0],
                               [1, 0, 0],
                               [0, 0, 1]],
                              [[1, 0, 0],
                               [0, 0, -1],
                               [0, 1, 0]],
                              [[1, 0, 0],
                               [0, 0, 1],
                               [0, -1, 0]]];
static ROTATION: [Matrix; 4] = [ID,
                                [[0, 0, 1],
                                 [0, 1, 0],
                                 [-1, 0, 0]],
                                [[-1, 0, 0],
                                 [0, 1, 0],
                                 [0, 0, -1]],
                                [[0, 0, -1],
                                 [0, 1, 0],
                                 [1, 0, 0]]];

fn translate(dest: &Beacon, src: &Beacon, scanner: &Scanner) -> Scanner {
    let deltas = [dest[0] - src[0],
                  dest[1] - src[1],
                  dest[2] - src[2]];

    //println!("trying translating {:?} to {:?} with deltas {:?}", src, dest, deltas);

    scanner.iter().map(|b|
                       [b[0] + deltas[0],
                        b[1] + deltas[1],
                        b[2] + deltas[2]]).collect()
}

fn merge(scanner: &mut Scanner, other: &Scanner) -> bool {
    for (rot, face) in ROTATION.iter().cartesian_product(FACING) {
        //println!("trying rotation {:?} and facing {:?}", rot, face);
        let rotated = other.iter().map(|b| transform(&transform(&b, &rot), &face)).collect();

        for dest in scanner.iter() {
            for src in &rotated {
                let potential_fit = translate(dest, src, &rotated);
                let matches = scanner.intersection(&potential_fit).count();
                //println!("attempted fit matches {}", matches);
                if matches >= 12 {
                    println!("merging fit into scanner");
                    scanner.extend(potential_fit);
                    return true;
                }

            }
        }
    }
    false
}

fn merge_all(scanners: &[Scanner]) -> Scanner {
    let mut merged = scanners[0].clone();
    let mut remaining = scanners.iter().skip(1).collect::<Vec<_>>();
    while !remaining.is_empty() {
        println!("{} remaining to merge", remaining.len());
        remaining = remaining.into_iter().filter(|scanner| !merge(&mut merged, scanner) ).collect();
    }
    merged
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Scanner>> {
    let mut scanners = vec![];
    for scanner in input.split("\n\n") {
        let mut beacons = Scanner::new();
        //skip the scanner # line, they're in order
        for beacon in scanner.lines().skip(1) {
            let coords = beacon.split(',').map(|n| n.parse()).collect::<Result<Vec<i32>,ParseIntError>>()?;
            match coords.try_into() {
                Ok(coords) => beacons.insert(coords),
                Err(_) => return Err(anyhow!("bad dimensions: {}", beacon))
            };
        }
        scanners.push(beacons);
    }
    Ok(scanners)
}

#[test]
fn test_parse_and_merge() {
    let scanners = parse_input("--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390").unwrap();
    let merged = merge_all(&scanners);

    let expected = HashSet::from([
        [459,-707,401],
        [-739,-1745,668],
        [-485,-357,347],
        [432,-2009,850],
        [528,-643,409],
        [423,-701,434],
        [-345,-311,381],
        [408,-1815,803],
        [534,-1912,768],
        [-687,-1600,576],
        [-447,-329,318],
        [-635,-1737,486]]);

    assert_eq!(merged.intersection(&expected).count(), expected.len());
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let scanners = parse_input(&input)?;
    let merged = merge_all(&scanners);

    println!("{}", merged.len());

    Ok(())
}
