use std::collections::VecDeque;
use itertools::Itertools;

fn rotate(circle: &mut VecDeque<u32>) {
    circle.rotate_left(1);
}

fn rotate_to(circle: &mut VecDeque<u32>, to: u32) {
    while circle.back().unwrap() != &to {
        rotate(circle);
    }
}

fn parse_input(s: &str) -> VecDeque<u32> {
    let mut circle = s.chars().map(|c| c.to_digit(10).unwrap() ).collect::<VecDeque<u32>>();
    //rotate current cup to tail of queue
    rotate(&mut circle);
    circle
}

fn find_destination(circle: &VecDeque<u32>) -> u32 {
    let current = circle.back().unwrap();
    let (greater, lesser): (Vec<_>, Vec<_>) = circle.iter().take(circle.len() - 1).partition(|n| *n > current);
    *lesser.iter().max().or(greater.iter().max()).unwrap()
}

fn make_move(circle: &mut VecDeque<u32>) {
    let current = *circle.back().unwrap();
    let removed = circle.drain(0..3).collect::<Vec<_>>();
    let destination = find_destination(circle);
    println!("after remove: circle: {:?} removed: {:?} current: {} destination: {}", circle, removed, current, destination);
    rotate_to(circle, destination);
    println!("{:?}", circle);
    circle.extend(removed);
    println!("{:?}", circle);
    rotate_to(circle, current);
    println!("{:?}", circle);
    rotate(circle);
    println!("end of move: circle: {:?}", circle);
}

fn make_moves(circle: &mut VecDeque<u32>, n: usize) {
    for i in 1..=n {
        println!("MOVE {}", i);
        make_move(circle);
        println!("");
    }
}

fn circle_string(circle: &VecDeque<u32>) -> String {
    let mut circle = circle.clone();
    rotate_to(&mut circle, 1);
    circle.iter().take(circle.len() - 1).join("")
}

#[test]
fn test_make_moves() {
    let mut circle = parse_input("389125467");
    make_moves(&mut circle, 10);
    assert_eq!(circle_string(&circle), "92658374");
}

fn main() {
    let mut circle = parse_input("253149867");
    make_moves(&mut circle, 100);
    println!("{}", circle_string(&circle));
}
