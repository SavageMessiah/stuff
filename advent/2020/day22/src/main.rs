use anyhow::Result;
use std::collections::VecDeque;

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

fn play_game(decks: &Vec<Deck>) -> (usize, Deck) {
    let mut decks = decks.clone();
    for round in 1.. {
        println!("Round {}", round);
        let mut cards = draw_cards(&mut decks);
        println!("Draws: {:?}", cards);
        let round_winner = cards.iter().enumerate().max_by_key(|(_, card)| *card).unwrap().0;
        println!("Round winner: {}", round_winner + 1);
        cards.sort();
        cards.reverse();
        decks[round_winner].extend(cards);

        if let Some(winner) = winner(&decks) {
            return winner
        }
    }
    unreachable!()
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
    let (player, deck) = play_game(&decks);
    assert_eq!(player, 1);
    assert_eq!(score_deck(&deck), 306);
    Ok(())
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let decks = parse_decks(&input)?;
    print_decks(&decks);
    let (player, deck) = play_game(&decks);

    println!("player {} won with {} points", player + 1, score_deck(&deck));
    Ok(())
}
