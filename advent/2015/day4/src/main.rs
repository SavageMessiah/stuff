const KEY: &str = "ckczppom";

fn is_coin(n: u32, prefix: &str) -> bool {
    let s = format!("{}{}", KEY, n);
    format!("{:x}", md5::compute(s)).starts_with(prefix)
}

fn coins(prefix: &str) -> impl Iterator<Item = u32> + '_ {
    (0..0xFFFFFFFFu32).filter(move |n| is_coin(*n, prefix))
}

fn main() {
    let first = coins("00000").nth(0).expect("no coins in u32?!");
    println!("First coin {}", first);
}
