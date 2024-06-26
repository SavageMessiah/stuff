use itertools::Itertools;
use std::iter::successors;

fn ascii_to_n(d: char) -> u64 {
    (d as u8 - 97) as u64
}

fn n_to_ascii(n: u64) -> char {
    (n as u8 + 97) as char
}

fn parse(s: &str) -> u64 {
    let mut n = 0;
    for (i, c) in s.chars().rev().enumerate() {
        n += 26u64.pow(i as u32) * ascii_to_n(c);
    }
    n
}

#[test]
fn test_parse() {
    assert_eq!(parse("a"), 0);
    assert_eq!(parse("ba"), 26);
    assert_eq!(parse("bb"), 27);
}

fn unparse(n: u64) -> String {
    let mut s = Vec::new();
    let mut n = n;
    while n > 0 {
        let rem = n % 26;
        n = n / 26;

        s.push(n_to_ascii(rem))
    }

    s.iter().rev().collect::<String>()
}

#[test]
fn test_unparse() {
    assert_eq!(unparse(3), "d");
    assert_eq!(unparse(26), "ba");
    assert_eq!(unparse(27), "bb");
}

fn inc(s: &str) -> String {
    unparse(parse(s) + 1)
}

fn separate_pairs(s: &str) -> bool {
    let mut found_pair = None;
    for (a, _b) in s.chars().tuple_windows().filter(|(a, b)| a == b) {
        match found_pair {
            Some(f) if f != a => {
                return true
            },
            None => {
                found_pair = Some(a);
            },
            _ => {}
        }
    }
    false
}

fn increasing_straight(s: &str) -> bool {
    s.chars().tuple_windows().find(|(a, b, c)| {
        let a = ascii_to_n(*a) + 2;
        let b = ascii_to_n(*b) + 1;
        let c = ascii_to_n(*c);
        a == b && b == c
    }).is_some()
}

fn no_confusing(s: &str) -> bool {
    s.chars().find(|&c| c == 'i' || c == 'o' || c == 'l').is_none()
}

fn valid(s: &str) -> bool {
    separate_pairs(s) && increasing_straight(s) && no_confusing(s)
}


fn main() {
    let next = successors(Some("cqjxjnds".to_string()), |s| Some(inc(s))).
        filter(|s| valid(s)).nth(1).unwrap();

    println!("{}", next);
}
