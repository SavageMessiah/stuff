use std::{cmp::{min, Ordering}, collections::BinaryHeap};

type Position = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: Position
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Map {
    costs: Vec<u8>,
    width: usize,
    height: usize
}

impl Map {
    fn hugify(&self, n: usize) -> Map {
        let mut map = Map {
            costs: vec![],
            width: self.width * n,
            height: self.height * n
        };

        for y in 0..map.height {
            for x in 0..map.width {
                let mut cost = self.cost_at((x % self.width, y % self.height)) as usize + (y / self.height) + (x / self.width);
                cost = (cost - 1) % 9 + 1;
                map.costs.push(cost as u8);
            }
        }
        map
    }


    fn cost_at(&self, pos: Position) -> u8 {
        self.costs[pos.1 * self.width + pos.0]
    }

    fn shortest_path(&self) -> usize {
        let goal = (self.width - 1, self.height - 1);
        let start = (0, 0);

        let mut distances = vec![usize::MAX; self.costs.len()];
        let mut heap = BinaryHeap::new();

        distances[0] = 0;
        heap.push(State { cost: 0, pos: start });

        while let Some(State { cost, pos }) = heap.pop() {
            if pos == goal {
                return cost;
            }

            //already found a shorter path
            if cost > distances[self.width * pos.1 + pos.0] {
                continue;
            }

            for neighbor in [(pos.0.saturating_sub(1), pos.1),
                             (min(pos.0 + 1, self.width - 1), pos.1),
                             (pos.0, pos.1.saturating_sub(1)),
                             (pos.0, min(pos.1 + 1, self.height - 1))] {
                if neighbor == pos {
                    continue; // we hit the edge
                }

                let idx = self.width * neighbor.1 + neighbor.0;
                let next = State {
                    cost: cost + self.cost_at(neighbor) as usize,
                    pos: neighbor
                };

                if next.cost < distances[idx] {
                    distances[idx] = next.cost;
                    heap.push(next);
                }

            }
        }

        unreachable!();
    }
}

fn parse_input(input: &str) -> Map {
    let mut map = Map {
        costs: vec![],
        width: 0,
        height: 0,
    };
    for l in input.lines() {
        for c in l.chars() {
            map.costs.push(c.to_digit(10).unwrap() as u8);
            if map.height == 0 {
                map.width += 1;
            }

        }
        map.height += 1;
    }
    map
}

#[test]
fn test_parse_apply_score() {
    let map = parse_input("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581").hugify(5);

    let path_cost = map.shortest_path();

    assert_eq!(path_cost, 315);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let map = parse_input(&input).hugify(5);
    let path_cost = map.shortest_path();

    println!("{}", path_cost);

    Ok(())
}
