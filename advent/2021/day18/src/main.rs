use std::{fmt::{Display, Write}, str::FromStr};
use winnow::{
    prelude::*,
    ascii::{digit1 as digits, space0 as spaces},
    combinator::{alt,
                 delimited,
                 separated_pair},
};

#[derive(Debug, Clone)]
enum SnailNum {
    Pair(Box<SnailNum>, Box<SnailNum>),
    Num(u32)
}

#[derive(Debug, Eq, PartialEq)]
enum ExplodeResult {
    None,
    Explode([u32; 2]),
    AddToLeft(u32),
    AddToRight(u32),
    Exploded
}

impl SnailNum {
    fn explode(&mut self, depth: usize) -> ExplodeResult {
        use SnailNum::*;
        use ExplodeResult::*;
        match self {
            Num(_) => None,
            Pair(bl, br) => match (&mut **bl, &mut **br) {
                (Num(l), Num(r)) if depth >= 4 => {
                    let pair = [*l, *r];
                    *self = Num(0);
                    println!("exploding: {:?}", pair);
                    Explode(pair)
                },
                (l, r) => {
                    let lex = l.explode(depth + 1);
                    println!("left explode result {:?}", lex);
                    match lex {
                        None => (),
                        Exploded => return Exploded,
                        Explode([ln, rn]) => {
                            r.add_on_left(rn);
                            return AddToLeft(ln);
                        },
                        AddToRight(n) => {
                            r.add_on_left(n);
                            return Exploded;
                        },
                        atl @ AddToLeft(_) => {
                            return atl;
                        }
                    }

                    let rex = r.explode(depth + 1);
                    println!("right explode result {:?}", rex);
                    match rex {
                        None => None,
                        Exploded => Exploded,
                        Explode([ln, rn]) => {
                            l.add_on_right(ln);
                            AddToRight(rn)
                        },
                        atr @ AddToRight(_) => atr,
                        AddToLeft(n) => {
                            l.add_on_right(n);
                            Exploded
                        }
                    }
                }
            }

        }
    }

    fn add_on_left(&mut self, n: u32) {
        use SnailNum::*;
        match self {
            Num(s) => *s += n,
            Pair(l, _) => l.add_on_left(n)
        }
    }

    fn add_on_right(&mut self, n: u32) {
        use SnailNum::*;
        match self {
            Num(s) => *s += n,
            Pair(_, r) => r.add_on_right(n)
        }
    }

    fn split(&mut self) -> bool {
        use SnailNum::*;
        match self {
            Num(n) if *n >= 10 => {
                let div = *n as f32 / 2.0;
                println!("splitting: {}", n);
                *self = Pair(Box::new(Num(div.floor() as u32)), Box::new(Num(div.ceil() as u32)));
                true
            },
            Pair(l, r) => {
                if l.split() {
                    true
                } else {
                    r.split()
                }
            },
            _ => false
        }
    }

    fn magnitude(&self) -> u32 {
        use SnailNum::*;
        match self {
            Num(n) => *n,
            Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    fn reduce(&mut self) {
        loop {
            if self.explode(0) != ExplodeResult::None {
                println!("after explode: {}", self);
                continue;
            }

            if self.split() {
                println!("after split: {}", self);
                continue;
            }

            break;
        }
    }
}

impl std::ops::Add for SnailNum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = SnailNum::Pair(Box::new(self), Box::new(rhs));
        println!("after addition: {}", res);
        res.reduce();
        println!("after reduce: {}", res);
        res
    }
}

fn pair_parser(i: &mut &str) -> PResult<SnailNum> {
    delimited('[',
              separated_pair(snailnum_parser,
                             delimited(spaces, ',', spaces),
                             snailnum_parser).map(|(l, r)| SnailNum::Pair(Box::new(l), Box::new(r))),
              ']').parse_next(i)
}

fn num_parser(i: &mut &str) -> PResult<SnailNum> {
    delimited(spaces,
              digits.try_map(FromStr::from_str).map(SnailNum::Num),
              spaces).parse_next(i)
}

fn snailnum_parser(i: &mut &str) -> PResult<SnailNum> {
    alt((pair_parser, num_parser)).parse_next(i)
}

impl Display for SnailNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SnailNum::*;
        match self {
            Num(n) => f.write_fmt(format_args!("{}", n)),
            Pair(l, r) => {
                f.write_char('[')?;
                l.fmt(f)?;
                f.write_char(',')?;
                r.fmt(f)?;
                f.write_char(']')
            }
        }
    }
}

impl FromStr for SnailNum {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match snailnum_parser.parse(s) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(anyhow::anyhow!("parse error: {}", err))
        }
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<SnailNum>> {
    input.lines().map(FromStr::from_str).collect()
}

#[test]
fn test_parse_and_add() {

    let tests = [("[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
                 ("[1,1]
[2,2]
[3,3]
[4,4]", "[[[[1,1],[2,2]],[3,3]],[4,4]]"),
    ("[1,1]
[2,2]
[3,3]
[4,4]
[5,5]", "[[[[3,0],[5,3]],[4,4]],[5,5]]"),
    ("[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]", "[[[[5,0],[7,4]],[5,5]],[6,6]]"),
    ("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
", "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")];

    for (input, out) in tests {
        println!("\n\ntesting: {}\n", input);
        let nums = parse_input(input).unwrap();

        let sum = nums.iter().cloned().reduce(|a, s| a + s).unwrap();

        assert_eq!(format!("{}", sum), out);
    }
}


fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let nums = parse_input(&input)?;
    let sum = nums.iter().cloned().reduce(|a, s| a + s).unwrap();

    println!("magnitude: {}", sum.magnitude());

    Ok(())
}
