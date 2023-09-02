use anyhow::Result;
use std::collections::{VecDeque, HashSet, HashMap};

type Card = u32;
type Deck = VecDeque<Card>;

fn winner(decks: &Vec<Deck>) -> Option<(usize, Deck)> {
    let potential_winners = decks.iter().enumerate().filter(|(_, deck)| !deck.is_empty()).collect::<Vec<_>>();
    if potential_winners.len() == 1 {
        let (player, deck) = potential_winners[0];
        Some((player, deck.clone()))
    } else {
        None
    }
}

fn draw_cards(decks: &mut Vec<Deck>) -> Vec<Card> {
    let mut cards = vec![];
    for deck in decks {
        cards.push(deck.pop_front().unwrap()); //shouldn't be calling this with empty decks
    }
    cards
}

fn round_winner(cards: &Vec<Card>, decks: &Vec<Deck>, game_memo: &mut HashMap<Vec<Deck>, (usize, Deck)>) -> usize {
    if cards.iter().enumerate().all(|(player, card)| decks[player].len() >= *card as usize) {
        println!("Recursing!");
        let mut sub_decks = vec![];
        for (player, card) in cards.iter().enumerate() {
            sub_decks.push(decks[player].iter().take(*card as usize).copied().collect::<Deck>());
        }
        return play_game(&sub_decks, game_memo).0
    }
    cards.iter().enumerate().max_by_key(|(_, card)| *card).unwrap().0
}

fn play_game(starting_decks: &Vec<Deck>, game_memo: &mut HashMap<Vec<Deck>, (usize, Deck)>) -> (usize, Deck) {
    if let Some(win) = game_memo.get(starting_decks) {
        println!("Previous game win: {:?}", win);
        return win.clone();
    }
    let mut previous_states = HashSet::new();
    let mut decks = starting_decks.clone();
    for round in 1.. {
        //println!("Round {}", round);
        if previous_states.contains(&decks) {
            return(0, decks[0].clone())
        }
        previous_states.insert(decks.clone());

        let cards = draw_cards(&mut decks);
        //println!("Draws: {:?}", cards);
        let round_winner = round_winner(&cards, &decks, game_memo);
        //println!("Round winner: {}", round_winner + 1);
        //annoying that it no longer obviously generalizes to n players
        decks[round_winner].push_back(cards[round_winner]);
        decks[round_winner].push_back(cards[(round_winner + 1) % 2]);

        if let Some(winner) = winner(&decks) {
            game_memo.insert(starting_decks.clone(), winner.clone());
            return winner
        }
    }
    unreachable!()
}

fn play(starting_decks: &Vec<Deck>) -> (usize, Deck) {
    let mut game_memo = HashMap::new();
    play_game(starting_decks, &mut game_memo)
}

fn score_deck(deck: &Deck) -> u32 {
    deck.iter().rev().enumerate().map(|(i, c)| (i as u32 + 1) * c).sum()
}

fn print_decks(decks: &Vec<Deck>) {
    for (player, deck) in decks.iter().enumerate() {
        print!("Player {}: ", player + 1);
        for card in deck.iter().rev() {
            println!("{}", card);
        }
    }
}

fn parse_decks(s: &str) -> Result<Vec<Deck>> {
    let mut decks = vec![];
    for player in s.split("\n\n") {
        let mut deck = VecDeque::new();
        for card in player.lines().skip(1) { //skip player number
            deck.push_back(card.parse()?);
        }
        decks.push(deck);
    }
    Ok(decks)
}

#[test]
fn test_play() -> Result<()> {
    let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
    let decks = parse_decks(input)?;
    let (player, deck) = play(&decks);
    assert_eq!(player, 1);
    assert_eq!(score_deck(&deck), 291);
    Ok(())
}

#[test]
fn test_loop() -> Result<()> {
    let input = "Player 1:
43
19

Player 2:
2
29
14";
    let decks = parse_decks(input)?;
    let (player, deck) = play(&decks);
    println!("deck: {:?}", deck);
    assert_eq!(player, 0);
    assert_eq!(score_deck(&deck), 105);
    Ok(())
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let decks = parse_decks(&input)?;
    print_decks(&decks);
    let (player, deck) = play(&decks);

    println!("player {} won with {} points", player + 1, score_deck(&deck));
    Ok(())
}
