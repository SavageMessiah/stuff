type Map = Vec<Vec<u32>>;

fn parse_input(s: &str) -> Map {
    s.lines().map(|l| {
        l.chars().map(|c| c.to_digit(10).unwrap() ).collect()
    }).collect()
}

#[derive(Eq, PartialEq)]
enum FillState {
    Basin(usize),
    Unknown,
    Rim
}

type BasinMap = Vec<Vec<FillState>>;

fn fill_basin(map: &mut BasinMap, x: usize, y: usize, basin: usize) -> u32 {
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    if map[y][x] != FillState::Unknown {
        return 0;
    }

    map[y][x] = FillState::Basin(basin);
    let x = x as i32;
    let y = y as i32;

    let mut basin_size = 1;
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        if (y == 0 && dy == -1) || (y + dy == height) || (x == 0 && dx == -1) || (x + dx == width) {
            continue;
        }
        basin_size += fill_basin(map, (x + dx) as usize, (y + dy) as usize, basin);
    }

    basin_size
}

fn basins(map: &Map) -> Vec<u32> {
    let mut basin_map: Vec<Vec<FillState>> = map.iter().map(|row| {
        row.iter().map(|c|
                       if *c == 9 {
                           FillState::Rim
                       } else {
                           FillState::Unknown
                       }).collect()
    }).collect();

    let mut basins = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if basin_map[y][x] == FillState::Unknown {
                basins.push(fill_basin(&mut basin_map, x, y, basins.len()));
            }
        }
    }
    basins
}

fn answer(basins: &Vec<u32>) -> u32 {
    let mut basins = basins.clone();
    basins.sort();
    basins.reverse();

    basins[0..3].iter().product()
}

#[test]
fn test_parse_and_answer() {
    let map = parse_input("2199943210
3987894921
9856789892
8767896789
9899965678");
    let basins = basins(&map);

    assert_eq!(answer(&basins), 1134);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let map = parse_input(&input);
    let basins = basins(&map);

    println!("{}", answer(&basins));

    Ok(())
}
