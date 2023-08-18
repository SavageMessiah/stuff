use anyhow::{anyhow, Result};

fn parse_busses(s: &str) -> Vec<(u64, u64)> {
    s.split(',')
     .map(|b| b.parse::<u64>().ok())
     .enumerate()
     .filter_map(|(i, b)| {
         match b {
             None => None,
             Some(b) => Some((i as u64, b))
         }
     }).collect::<Vec<_>>()
}

fn find_timestamp(busses: &Vec<(u64, u64)>) -> Option<u64> {
    let first = busses[0].1;
    'outer: for n in 0.. {
        let time = first * n;
        for (offset, b) in busses.iter().skip(1) {
            if (time + *offset) % b != 0 {
                continue 'outer;
            }
        }

        return Some(time);
    }
    None
}

fn find_timestamp_not_shit(busses: &Vec<(u64, u64)>) -> Option<u64> {
    let mut crt = busses.iter().copied().map(|(i, b)| (if i == 0 { 0 } else { b - (i % b) }, b)).collect::<Vec<_>>();
    crt.sort_by_key(|(_, n)| *n);
    crt.reverse();
    println!("{:?}", crt);
    let (mut n, mut modulo) = crt[0];
    for (rem, m) in crt.iter().skip(1) {
        while n % m != *rem {
            n += modulo;
        }
        modulo *= m;
    }
    Some(n)
}

#[test]
fn test_find_timestamp() {
    let examples = [("7,13,x,x,59,x,31,19",1068781),
                    ("17,x,13,19",3417),
                    ("67,7,59,61",754018),
                    ("67,x,7,59,61",779210),
                    ("67,7,x,59,61",1261476),
                    ("1789,37,47,1889",1202161486)];
    for (s, ex) in examples {
        let busses = parse_busses(s);
        let t = find_timestamp_not_shit(&busses);
        assert_eq!(t, Some(ex));
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let busses = parse_busses(input.lines().nth(1).ok_or(anyhow!("missing second line"))?);
    println!("{:?}", busses);
    let answer = find_timestamp_not_shit(&busses);
    println!("answer: {:?}", answer);

    Ok(())
}
