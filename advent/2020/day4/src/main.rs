use std::collections::HashMap;

#[derive(Debug)]
struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

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
        REQUIRED_FIELDS.iter().all(|f| self.fields.contains_key(*f))
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let answer = input.split("\n\n").map(Passport::parse).filter(Passport::is_valid).count();

    println!("answer {}", answer);

    Ok(())
}
