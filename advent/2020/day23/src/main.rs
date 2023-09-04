struct Circle {
    current: usize,
    max: usize,
    all: Vec<usize>,
    remove_buf: [usize; 3]
}

fn find_destination(current: usize, max: usize, removed: &[usize; 3]) -> usize {
    let mut dest = current - 1;
    while removed.contains(&dest) {
        dest = dest - 1;
    }
    if dest == 0 {
        return find_destination(max + 1, max, removed);

    }
    dest
}

impl Circle {
    fn new(contents: &[usize]) -> Circle {
        let current = contents[0];
        let mut all = vec![0usize; contents.len()];
        let mut last = current;
        for n in contents.iter().skip(1) {
            all[last - 1] = *n;
            last = *n;
        }
        all[last - 1] = current;
        let max = *all.iter().max().unwrap();

        Circle {
            all, current, max, remove_buf: [0; 3]
        }
    }

    fn next(&self, n: usize) -> usize {
        self.all[n - 1]
    }

    fn set_next(&mut self, n: usize, next: usize) {
        self.all[n - 1] = next;
    }

    fn make_move(&mut self) {
        let mut next = self.current;
        //get the next 3 after current
        for i in 0..3 {
            next = self.next(next);
            self.remove_buf[i] = next;
        }
        //slice them out of the ring
        self.set_next(self.current, self.next(next));
        let dest = find_destination(self.current, self.max, &self.remove_buf);

        //splice them back in
        let end = self.next(dest);
        self.set_next(dest, self.remove_buf[0]);
        self.set_next(self.remove_buf[2], end);

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

    fn result_product(&self) -> usize {
        let first = self.next(1);
        let second = self.next(first);
        first * second
    }
}

fn parse_input(s: &str) -> Vec<usize> {
    s.chars().map(|c| c.to_digit(10).unwrap() as usize ).collect()
}

#[test]
fn test_make_moves() {
    let mut circle = Circle::new(&parse_input("389125467"));
    circle.make_moves(10);
    assert_eq!(circle.result_string(), "92658374");
}

fn main() {
    let mut parsed = parse_input("253149867");
    parsed.extend(10usize..=1000000);
    let mut circle = Circle::new(&parsed);
    circle.make_moves(10000000);
    println!("{}", circle.result_product());
}
