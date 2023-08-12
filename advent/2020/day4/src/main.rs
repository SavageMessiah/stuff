use std::{collections::HashMap, ops::Range};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref HEIGHT: Regex = Regex::new(r"^([0-9]+)(cm|in)$").expect("bad regex");
    static ref COLOR: Regex = Regex::new(r"^#[0-9a-f]{6}$").expect("bad regex");
    static ref PID: Regex = Regex::new(r"^[0-9]{9}$").expect("bad regex");
    static ref YEAR: Regex = Regex::new(r"^[0-9]{4}$").expect("bad regex");
}

#[derive(Debug)]
struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

fn is_valid_year(s: &str, range: Range<u32>) -> bool {
    if !YEAR.is_match(s) {
        return false;
    }
    let Ok(year) = s.parse::<u32>() else { return false };
    range.contains(&year)
}

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

const REQUIRED_FIELDS: [(&str, fn(&str) -> bool); 7] = [
    ("byr", |v| is_valid_year(v, 1920..2003)),
    ("iyr", |v| is_valid_year(v, 2010..2021)),
    ("eyr", |v| is_valid_year(v, 2020..2031)),
    ("hgt", |v| {
        let Some(caps) = HEIGHT.captures(v) else { return false };
        let Ok(n) = caps[1].parse::<u32>() else { return false };
        match &caps[2] {
            "cm" if n >= 150 && n <= 193 => true,
            "in" if n >= 59 && n <= 76 => true,
            _ => false
        }
    }),
    ("hcl", |v| COLOR.is_match(v)),
    ("ecl", |v| EYE_COLORS.iter().any(|c| *c == v)),
    ("pid", |v| PID.is_match(v))];

impl<'a> Passport<'a> {
    fn parse(record: &'a str) -> Passport {
        let mut pp = Passport {
            fields: HashMap::new()
        };
        for pair in record.split_whitespace() {
            match pair.split_once(':') {
                None => continue,
                Some((field, val)) => pp.fields.insert(field, val)
            };
        }

        pp
    }

    fn is_valid(&self) -> bool {
        REQUIRED_FIELDS.iter().all(|(field, valid)| {
            match self.fields.get(field) {
                Some(v) if valid(v) => {
                    println!("valid f: {} v: {}", field, v);
                    true
                },
                Some(v) => {
                    println!("invalid f: {} v: {}", field, v);
                    false
                }
                _ => {
                    println!("missing: {}", field);
                    false
                }
            }
        })
    }
}

fn count_valid_passports(s: &str) -> usize {
    s.split("\n\n").map(Passport::parse).filter(Passport::is_valid).count()
}

#[test]
fn test_count_valid_passports() {
    assert_eq!(count_valid_passports("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"), 0);

    assert_eq!(count_valid_passports("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"), 4);
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let answer = count_valid_passports(input.as_str());

    println!("answer {}", answer);

    Ok(())
}
