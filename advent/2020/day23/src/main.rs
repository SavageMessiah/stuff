use std::collections::HashMap;

struct Circle {
    current: u64,
    max: u64,
    all: HashMap<u64, u64>,
    remove_buf: [u64; 3]
}

fn find_destination2(current: u64, max: u64, removed: &[u64; 3]) -> u64 {
    let mut dest = current - 1;
    while removed.contains(&dest) {
        dest = dest - 1;
    }
    if dest == 0 {
        return find_destination2(max + 1, max, removed);

    }
    dest
}

impl Circle {
    fn new(contents: impl IntoIterator<Item = u64>) -> Circle {
        let mut iter = contents.into_iter();
        let current = iter.next().unwrap();
        let mut all = HashMap::new();
        let mut last = current;
        for n in iter {
            all.insert(last, n);
            last = n;
        }
        all.insert(last, current);
        let max = *all.keys().max().unwrap();

        Circle {
            all, current, max, remove_buf: [0; 3]
        }
    }

    fn next(&self, n: u64) -> u64 {
        *self.all.get(&n).unwrap()
    }

    fn make_move(&mut self) {
        let mut next = self.current;
        //get the next 3 after current
        for i in 0..3 {
            next = self.next(next);
            self.remove_buf[i] = next;
        }
        //slice them out of the ring
        self.all.insert(self.current, self.next(next));
        let dest = find_destination2(self.current, self.max, &self.remove_buf);

        //splice them back in
        let end = self.next(dest);
        self.all.insert(dest, self.remove_buf[0]);
        self.all.insert(self.remove_buf[2], end);

        self.current = self.next(self.current);
    }

    fn make_moves(&mut self, n: usize) {
        for i in 1..=n {
            self.make_move()
        }
    }

    fn result_string(&self) -> String {
        let mut s = "".to_string();
        let mut next = self.next(1);
        while next != 1 {
            s.push_str(next.to_string().as_str());
            next = self.next(next);
        }
        s
    }

    fn result_product(&self) -> u64 {
        let first = self.next(1);
        let second = self.next(first);
        first * second
    }
}

fn parse_input(s: &str) -> Vec<u64> {
    s.chars().map(|c| c.to_digit(10).unwrap() as u64 ).collect()
}


#[test]
fn test_make_moves() {
    let mut circle = Circle::new(parse_input("389125467"));
    circle.make_moves(10);
    assert_eq!(circle.result_string(), "92658374");
}

fn main() {
    let parsed = parse_input("253149867");
    let mut circle = Circle::new(parsed.iter().copied().chain(10u64..=1000000));
    circle.make_moves(10000000);
    println!("{}", circle.result_product());
}
