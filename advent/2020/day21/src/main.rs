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

struct Counts<'a> {
    allergen_food_counts: HashMap<&'a String,usize>,
    ingredient_allergen_counts: HashMap<&'a String,HashMap<&'a String, usize>>
}

fn allergen_counts(foods: &[Food]) -> Counts {
    let allergen_food_counts = foods.iter().flat_map(|f| f.allergens.iter()).counts();
    let mut ingredient_allergen_counts = HashMap::new();
    for f in foods {
        for i in &f.ingredients {
            let ingredient_allergens = ingredient_allergen_counts.entry(i).or_insert_with(HashMap::new);
            for a in &f.allergens {
                *ingredient_allergens.entry(a).or_insert(0) += 1;
            }
        }
    }
    Counts {
        allergen_food_counts, ingredient_allergen_counts
    }
}

fn partition_ingredients<'a>(counts: &Counts<'a>) -> (HashSet<&'a String>, HashSet<&'a String>) {
    let mut no_allergen_ingredients = HashSet::new();
    let mut allergen_ingredients = HashSet::new();
    'outer: for (i, allergen_counts) in &counts.ingredient_allergen_counts {
        for (allergen, count) in allergen_counts {
            println!("ingredient {} is in {} foods with {} which is in {} foods", i, count, &allergen, counts.allergen_food_counts.get(allergen).unwrap());
            if count == counts.allergen_food_counts.get(allergen).unwrap() {
                allergen_ingredients.insert(*i);
                continue 'outer;
            }
        }
        no_allergen_ingredients.insert(*i);
    }

    (allergen_ingredients, no_allergen_ingredients)
}

fn identify_allergens(allergens: &HashSet<&String>, counts: &Counts) -> HashMap<String, String> {
    let mut identified_allergens = HashMap::new();
    let mut unidentified_allergen_ingredients = allergens.clone();
    while identified_allergens.len() < allergens.len() {
        for ingredient in allergens {
            // looping over allergens and excluding identified ones because we need to modify
            // unidentified_allergen_ingredients
            if !unidentified_allergen_ingredients.contains(ingredient) {
                continue;
            }
            println!("ingredient: {}", ingredient);
            let potential_allergens = counts.ingredient_allergen_counts.get(ingredient).unwrap().iter()
                                                                       .filter_map(|(a, c)| {
                                                                           if c == counts.allergen_food_counts.get(a).unwrap() &&
                                                                              !identified_allergens.contains_key(*a) {
                                                                                  Some(a)
                                                                              } else {
                                                                                  None
                                                                              }
                                                                       }).collect::<Vec<_>>();
            println!("potential allergens: {:?}", potential_allergens);
            if potential_allergens.len() == 1 {
                let allergen = potential_allergens[0];
                println!("identified {} as {}", ingredient, allergen);
                identified_allergens.insert(allergen.to_string(), ingredient.to_string());
                unidentified_allergen_ingredients.remove(ingredient);
            }
        }
    }

    identified_allergens
}

fn count_ingredients(foods: &[Food], ingredients: &HashSet<&String>) -> usize {
    foods.iter().flat_map(|f| f.ingredients.iter()).filter(|i| ingredients.contains(*i)).count()
}

#[test]
fn test_count_no_allergen_ingredients() {
    let foods = parse_foods("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)");
    let counts = allergen_counts(&foods);
    let (ai, nai) = partition_ingredients(&counts);

    assert_eq!(ai, HashSet::from([&"sqjhc".to_string(), &"fvjkl".to_string(), &"mxmxvkd".to_string()]));
    assert_eq!(nai, HashSet::from([&"kfcds".to_string(), &"nhms".to_string(), &"sbzzf".to_string(), &"trh".to_string()]));
    assert_eq!(count_ingredients(&foods, &nai), 5);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let foods = parse_foods(&input);
    let counts = allergen_counts(&foods);
    let (ai, nai) = partition_ingredients(&counts);

    let identified = identify_allergens(&ai, &counts);
    let answer = identified.iter().sorted_by_key(|kv| kv.0).map(|kv| kv.1).join(",");

    println!("answer: {}", answer);
}
