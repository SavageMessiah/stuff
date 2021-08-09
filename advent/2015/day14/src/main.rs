use anyhow::{anyhow, Result};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
enum State {
    Flying,
    Resting,
}

#[derive(Debug)]
struct ReindeerDef {
    name: String,
    speed: u32,
    flight_time: u32,
    rest_time: u32,
}

#[derive(Debug)]
struct ReindeerSim<'a> {
    def: &'a ReindeerDef,
    remaining: u32,
    flown: u32,
    points: u32,
    state: State,
}

fn parse(s: &str) -> Result<Vec<ReindeerDef>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<name>\w+) can fly (?P<speed>\d+) km/s for (?P<flight>\d+) seconds, but then must rest for (?P<rest>\d+)").unwrap();
    }
    s.lines().map(|l| {
        let caps = RE.captures(l).ok_or(anyhow!("no match"))?;
        let flight_time = caps["flight"].parse()?;

        Ok(ReindeerDef {
            name: caps["name"].to_string(),
            speed: caps["speed"].parse()?,
            flight_time,
            rest_time: caps["rest"].parse()?,
        })
    }).collect()
}

fn advance(state: &mut Vec<ReindeerSim>) {
    for reindeer in state.iter_mut() {
        reindeer.remaining -= 1;
        match reindeer.state {
            State::Flying => {
                reindeer.flown += reindeer.def.speed;
                if reindeer.remaining == 0 {
                    reindeer.state = State::Resting;
                    reindeer.remaining = reindeer.def.rest_time;
                }
            },
            State::Resting => {
                if reindeer.remaining == 0 {
                    reindeer.state = State::Flying;
                    reindeer.remaining = reindeer.def.flight_time;
                }
            }
        }
    }
}

fn award_points(state: &mut Vec<ReindeerSim>) {
    let lead_distance = state.iter().map(|r| r.flown).max().unwrap();

    state.iter_mut().filter(|r| r.flown == lead_distance).for_each(|leader| {
        leader.points += 1;
    });
}

fn sim(defs: &Vec<ReindeerDef>, time: u32) -> Vec<ReindeerSim> {
    let mut state = defs.iter().map(|def| ReindeerSim {
        def,
        remaining:
        def.flight_time,
        flown: 0,
        points: 0,
        state: State::Flying}).collect::<Vec<_>>();

    for _ in 0..time {
        advance(&mut state);
        award_points(&mut state);
    }
    state
}


fn main() -> Result<()> {
    let defs = parse(include_str!("input.txt"))?;
    let res = sim(&defs, 2503);
    let winner = res.iter().max_by_key(|reindeer| reindeer.points).unwrap();

    println!("Winner is {} at {} points", winner.def.name, winner.points);

    Ok(())
}
