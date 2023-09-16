type Map = Vec<Vec<u32>>;

fn parse_input(s: &str) -> Map {
    s.lines().map(|l| {
        l.chars().map(|c| c.to_digit(10).unwrap() ).collect()
    }).collect()
}

fn low_points(map: &Map) -> Vec<u32> {
    let mut low_points = vec![];
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    for y in 0..height {
        'x: for x in 0..width {
            let floor_height = map[y as usize][x as usize];
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if (y == 0 && dy == -1) || (y + dy == height) || (x == 0 && dx == -1) || (x + dx == width) {
                    continue;
                }
                if map[(y + dy) as usize][(x + dx) as usize] <= floor_height {
                    continue 'x;
                }
            }
            low_points.push(floor_height);
        }
    }
    low_points
}

fn total_risk(low_points: &Vec<u32>) -> u32 {
    low_points.iter().map(|lp| lp + 1).sum()
}

#[test]
fn test_parse_and_risk() {
    let map = parse_input("2199943210
3987894921
9856789892
8767896789
9899965678");
    let low_points = low_points(&map);

    assert_eq!(total_risk(&low_points), 15);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let map = parse_input(&input);
    let low_points = low_points(&map);

    println!("{}", total_risk(&low_points));

    Ok(())
}
