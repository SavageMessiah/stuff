use std::num::ParseIntError;

use anyhow::{anyhow, Result};
use lazy_regex::regex;

#[derive(Clone, Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_line(s: &str) -> Result<Ingredient> {
    let name = s.split(":").next().ok_or(anyhow!("no name"))?;
    let params = regex!(r"-?\d+").find_iter(s).map(|m| m.as_str().parse::<i32>()).collect::<Result<Vec<_>, ParseIntError>>()?;

    Ok(Ingredient {
        name: name.to_string(),
        capacity: params[0],
        durability: params[1],
        flavor: params[2],
        texture: params[3],
        calories: params[4],
    })
}

fn combinations<'a, F>(ingredients : &'a [Ingredient], mut cb: F)
    where F: FnMut(Vec<(&'a Ingredient, i32)>)
{
    fn inner<'a, F>(ingredients: &'a [Ingredient], picks: Vec<(&'a Ingredient,i32)>, cb: &mut F)
        where F: FnMut(Vec<(&'a Ingredient, i32)>)
    {
        let so_far: i32 = picks.iter().map(|i| i.1).sum();
        if ingredients.is_empty() {
            if so_far == 100 {
                cb(picks);
            }
            return
        }

        let remaining = 100 - so_far;

        for i in (0..=remaining).rev() {
            let last_pos = ingredients.len() -1;
            let next_ingredient = &ingredients[last_pos];

            let mut picks = picks.clone();
            picks.push((next_ingredient, i));

            inner(&ingredients[..(last_pos)], picks, cb);
        }
    }

    inner(ingredients, vec![], &mut cb)
}

fn clamp_to_zero(n: &mut i32) {
    if *n < 0 {
        *n = 0;
    }
}

fn score(picks: &[(&Ingredient, i32)]) -> i32 {
    let mut totals = Ingredient {
        name: "Recipe".to_string(),
        capacity: 0,
        durability: 0,
        flavor: 0,
        texture: 0,
        calories: 0,
    };

    for (ingredient, amount) in picks {
        totals.capacity += ingredient.capacity * amount;
        totals.durability += ingredient.durability * amount;
        totals.flavor += ingredient.flavor * amount;
        totals.texture += ingredient.texture * amount;
    }

    clamp_to_zero(&mut totals.capacity);
    clamp_to_zero(&mut totals.durability);
    clamp_to_zero(&mut totals.flavor);
    clamp_to_zero(&mut totals.texture);

    totals.capacity * totals.durability * totals.flavor * totals.texture
}

#[test]
fn test_score() {
    let ingredients = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".lines().map(parse_line).collect::<Result<Vec<Ingredient>>>().unwrap();

    let combination = vec![(&ingredients[0], 44), (&ingredients[1], 56)];

    assert_eq!(score(&combination), 62842880);
}

fn main() -> Result<()>{
    let ingredients = include_str!("input.txt").lines().map(parse_line).collect::<Result<Vec<Ingredient>>>()?;

    let mut best: Option<(Vec<(&Ingredient, i32)>, i32)> = None;

    combinations(&ingredients, |ins| {
        let score = score(&ins);
        match best {
            None => { best = Some((ins, score)) },
            Some((_, prev)) if prev < score => { best = Some((ins, score)) },
            _ => {},
        }
    });

    dbg!(best.unwrap());

    Ok(())
}
