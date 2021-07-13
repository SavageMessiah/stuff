fn three_vowels(s: &str) -> bool {
    let mut vowels = 0;
    //filter?
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowels += 1;
                if vowels >= 3 {
                    return true;
                }
            },
            _ => (),
        }
    }
    false
}

fn doubled_char(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    for pair in chars.windows(2) {
        if pair[0] == pair[1] {
            return true;
        }
    }
    false
}

fn no_bad_strings(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    for pair in chars.windows(2) {
        if pair == ['a', 'b'] ||
            pair == ['c', 'd'] ||
            pair == ['p', 'q'] ||
            pair == ['x', 'y'] {
                return false
            }
    }
    true
}

fn nice(s: &str) -> bool {
    three_vowels(s) && doubled_char(s) && no_bad_strings(s)
}

#[test]
fn nice_test() {
    assert!(nice("ugknbfddgicrmopn"));
    assert!(nice("aaa"));
    assert!(!nice("pq"));
    assert!(!nice("xy"));
    assert!(!nice("jchzalrnumimnmhp"));
    assert!(!nice("haegwjzuvuyypxyu"));
    assert!(!nice("dvszwmarrgswjxmb"));
}

fn main() {
    let input = include_str!("input.txt");
    let nice = input.lines().
        filter(|l| nice(l)).
        count();
    println!("Nice: {}", nice);
}
