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

fn split_repeat(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    for triple in chars.windows(3) {
        if triple[0] == triple[2] {
            return true;
        }
    }
    false
}

fn separate_pairs(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    for (i, pair) in chars.windows(2).enumerate() {
        for new_pair in chars[i+2..].windows(2) {
            if pair == new_pair {
                return true;
            }
        }
    }
    false
}

fn nice2(s: &str) -> bool {
    split_repeat(s) && separate_pairs(s)
}

#[test]
fn nice2_test() {
    assert!(split_repeat("xyx"));
    assert!(split_repeat("abcdefeghi"));
    assert!(split_repeat("aaa"));

    assert!(separate_pairs("aabcdefgaa"));
    assert!(separate_pairs("xyxy"));
    assert!(separate_pairs("xyaxy"));
    assert!(!separate_pairs("aaa"));

    assert!(nice2("qjhvhtzxzqqjkmpb"));
    assert!(nice2("xxyxx"));
    assert!(!nice2("uurcxstgmygtbstg"));
    assert!(!nice2("ieodomkazucvgmuy"));
}

fn main() {
    let input = include_str!("input.txt");
    let nice = input.lines().
        filter(|l| nice(l)).
        count();
    let nice2 = input.lines().
        filter(|l| nice2(l)).
        count();
    println!("Nice: {} Nice 2: {}", nice, nice2);
}
