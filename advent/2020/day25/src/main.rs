
fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut n = 1;
    for _ in 1..=loop_size {
        n = (subject_number * n) % 20201227;
    }
    n
}

fn loop_size(public_key: u64) -> u64 {
    let mut n = 1;
    for i in 1.. {
        n = (7 * n) % 20201227;
        if n == public_key {
            return i;
        }
    }
    unreachable!();
}

#[test]
fn test_loop_size() {
    assert_eq!(loop_size(5764801), 8);
    assert_eq!(loop_size(17807724), 11);
}

#[test]
fn test_transform() {
    assert_eq!(transform(17807724, 8), 14897079);
    assert_eq!(transform(5764801, 11), 14897079);
}

fn main() {
    //there is no part 2
    let keys = [11562782u64, 18108497];

    let loop_size = loop_size(keys[0]);

    println!("{}", transform(keys[1], loop_size));
}
