use std::{collections::HashMap, iter::repeat};

use itertools::Itertools;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Game {
    scores: [u32; 2],
    places: [u32; 2],
}

enum Turn {
    Game(Game),
    Win(usize)
}

impl Game {
    fn start(starts: [u32; 2]) -> Game {
        Game {
            scores: [0, 0],
            places: [starts[0] - 1, starts[1] - 1],
        }
    }

    fn turn(&self, player: usize, roll: u32) -> Turn {
        let mut succ = self.clone();
        let place = &mut succ.places[player];
        *place = (*place + roll) % 10;

        let score = &mut succ.scores[player];
        *score += *place + 1;
        if *score >= 21 {
            Turn::Win(player)
        } else {
            Turn::Game(succ)
        }
    }
}

fn play_all(starts: [u32; 2]) -> [u64; 2] {
    let mut wins = [0, 0];
    let mut universes = HashMap::from([(Game::start(starts), 1u64)]);
    let mut player = 0;
    while !universes.is_empty() {
        let mut next_universes = HashMap::new();

        for dice in repeat([1, 2, 3]).take(3).multi_cartesian_product() {
            let roll = dice.iter().sum();
            println!("roll: {:?} total: {}", dice, roll);

            for (game, count) in &universes {
                print!("{:?} {} results in", game, count);
                match game.turn(player, roll) {
                    Turn::Game(game) => {
                        println!(" {:?}", game);
                        *next_universes.entry(game).or_insert(0u64) += count
                    },
                    Turn::Win(player) => {
                        println!(" {} wins", player);
                        wins[player] += count
                    }
                }
            }
        }

        player = (player + 1) % 2;
        universes = next_universes;
    }
    wins
}

#[test]
fn test_play() {
    assert_eq!(play_all([4, 8]), [444356092776315, 341960390180808]);
}


fn main() {
    let wins = play_all([10, 4]);

    println!("{}", wins.iter().max().unwrap());
}
