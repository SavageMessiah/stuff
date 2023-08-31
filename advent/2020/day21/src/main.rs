use std::{str::FromStr, collections::{HashSet, HashMap}};

use itertools::Itertools;

struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>
}

impl FromStr for Food {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ingredients, allergens) = s.split_once(" (contains ").unwrap_or((s, ""));
        Ok(Food {
            ingredients: ingredients.split(" ").map(|s| s.to_string()).collect(),
            allergens: allergens.trim_end_matches(")").split(", ").map(|s| s.to_string()).collect()
        })
    }
}

fn parse_foods(s: &str) -> Vec<Food> {
    s.lines().map(|l| l.parse()).collect::<Result<Vec<_>, _>>().unwrap()
}

fn no_allergen_ingredients(foods: &[Food]) -> HashSet<String> {
    let mut allergen_food_counts = foods.iter().flat_map(|f| f.allergens.iter()).counts();
    let mut ingredient_allergen_counts = HashMap::new();
    for f in foods {
        for i in &f.ingredients {
            let ingredient_allergens = ingredient_allergen_counts.entry(i).or_insert_with(HashMap::new);
            for a in &f.allergens {
                *ingredient_allergens.entry(a).or_insert(0) += 1;
            }
        }
    }
    let mut no_allergen_ingredients = HashSet::new();
    'outer: for (i, allergen_counts) in ingredient_allergen_counts {
        for (allergen, count) in allergen_counts {
            println!("ingredient {} is in {} foods with {} which is in {} foods", i, count, &allergen, allergen_food_counts.get(&allergen).unwrap());
            if count == *allergen_food_counts.get(&allergen).unwrap() {
                continue 'outer;
            }
        }
        no_allergen_ingredients.insert(i.clone());
    }

    no_allergen_ingredients
}

fn count_ingredients(foods: &[Food], ingredients: &HashSet<String>) -> usize {
    foods.iter().flat_map(|f| f.ingredients.iter()).filter(|i| ingredients.contains(*i)).count()
}

#[test]
fn test_count_no_allergen_ingredients() {
    let foods = parse_foods("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)");
    let nai = no_allergen_ingredients(&foods);
    assert_eq!(nai, HashSet::from(["kfcds".to_string(), "nhms".to_string(), "sbzzf".to_string(), "trh".to_string()]));
    assert_eq!(count_ingredients(&foods, &nai), 5);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let foods = parse_foods(&input);
    let nai = no_allergen_ingredients(&foods);

    let answer = count_ingredients(&foods, &nai);
    println!("answer: {}", answer);
}
