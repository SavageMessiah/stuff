struct Game {
    scores: [u32; 2],
    places: [u32; 2],
    die: u32,
    rolls: u32
}

impl Game {
    fn roll(&mut self) -> u32 {
        self.rolls += 1;
        let val = self.die;
        self.die += 1;
        if self.die > 100 {
            self.die = 1;
        }
        val
    }
    fn rolln(&mut self, n: usize) -> u32 {
        let mut total = 0;
        for _ in 0..n {
            total += self.roll();
        }
        total
    }
    fn step(&mut self, player: usize, n: u32) -> u32 {
        let place = &mut self.places[player];
        for _ in 0..n {
            *place += 1;
            if *place > 10 {
                *place = 1;
            }
        }
        *place
    }
    fn turn(&mut self, player: usize) -> bool {
        let roll = self.rolln(3);
        self.scores[player] += self.step(player, roll);

        self.scores[player] >= 1000
    }
    fn play(starts: [u32; 2]) -> u32 {
        let mut game = Game {
            scores: [0, 0],
            places: starts,
            die: 1,
            rolls: 0
        };
        let mut player = 0;
        while !game.turn(player) {
            player = (player + 1) % 2;
        }
        println!("Player {} won with {}", player + 1, game.scores[player]);

        game.scores[(player + 1) % 2] * game.rolls
    }
}

#[test]
fn test_play() {
    assert_eq!(Game::play([4,8]), 739785);
}


fn main() {
    let input = [10, 4];
    println!("{}", Game::play(input));
}
