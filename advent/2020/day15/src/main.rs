use std::collections::HashMap;

use anyhow::Result;

type Turn = u64;

#[derive(Debug)]
struct State {
    last_num: u64,
    last_seen: Option<Turn>,
    seen: HashMap<u64, Turn>
}

impl State {
    fn new() -> State {
        State {
            last_num: 0,
            last_seen: None,
            seen: HashMap::new()
        }
    }

    fn say(&mut self, turn: Turn, n: u64) {
        println!("turn: {} saying: {}", turn, n);
        self.last_seen = self.seen.insert(n, turn);
        self.last_num = n;
    }

    fn play(&mut self, input: &Vec<u64>, turns: Turn) {
        let mut iter = input.iter();
        for turn in 1..=turns {
            if let Some(n) = iter.next() {
                self.say(turn, *n);
                continue;
            }
            match self.last_seen {
                None => self.say(turn, 0),
                Some(last_turn) => self.say(turn, turn - 1 - last_turn)
            }
        }
    }
}

#[test]
fn test_play() {
    let tests = [(vec![0, 3, 6], 436),
                 (vec![1, 3, 2], 1),
                 (vec![2, 1, 3], 10),
                 (vec![1, 2, 3], 27),
                 (vec![2, 3, 1], 78),
                 (vec![3, 2, 1], 438),
                 (vec![3, 1, 2], 1836),
    ];
    for (input, ex) in tests {
        let mut s = State::new();
        println!("test {:?}", input);
        s.play(&input, 2020);
        assert_eq!(s.last_num, ex);
    }
}

fn main() -> Result<()> {
    let input = "2,0,1,7,4,14,18";
    let numbers = input.split(',').map(|n| n.parse()).collect::<Result<Vec<u64>, _>>()?;
    let mut s = State::new();
    s.play(&numbers, 2020);
    let answer = s.last_num;
    println!("answer: {:?}", answer);

    Ok(())
}
