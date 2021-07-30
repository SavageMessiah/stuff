#![feature(slice_group_by)]

use itertools::unfold;

fn look_and_say(s: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    for run in s.group_by(|a, b| *a == *b) {
        res.push(run.len() as u8);
        res.push(run[0]);
    }

    res
}

#[test]
fn test_lns() {
    assert_eq!(look_and_say(&vec![1]), vec![1, 1]);
    assert_eq!(look_and_say(&vec![1, 1]), vec![2, 1]);
    assert_eq!(look_and_say(&vec![2, 1]), vec![1, 2, 1, 1]);

}

fn main() {
    let res = unfold(vec![3, 1, 1, 3, 3, 2, 2, 1, 1, 3], |v| {
        *v = look_and_say(&v);
        Some(v.clone())
    }).take(50).last().unwrap();

    println!("len = {}", res.len());
}
