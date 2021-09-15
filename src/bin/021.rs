use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../inputs/021.txt");
    let foods = parse_input(input);

    get_allergic_ingredients(&foods);
}

#[derive(Debug)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

fn parse_input(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(|line| {
            let (ingredients, allergens) = line.split_once(" (contains ").unwrap();
            Food {
                ingredients: ingredients.split(' ').collect(),
                allergens: allergens.trim_end_matches(')').split(", ").collect(),
            }
        })
        .collect()
}

fn get_allergic_ingredients<'a>(foods: &[Food]) -> HashMap<&'a str, &'a str> {
    // first find the list of an allergen and all the ingerdients it could be in
    // to do this go through the list and each time an allergen appears, check the list of ingredients
    // for that allergen
    // if none exist then put the whole list of ingredients in
    // once it is filled subsequent ones should update the value by findining the intersection of the current ingredients
    // and the new possible ones in the list
    let allergen_ingredients: HashMap<&str, HashSet<&str>> = foods
        .iter()
        .flat_map(|food| food.allergens.iter().map(move |allergen| (food, allergen)))
        .fold(HashMap::new(), |mut map, (food, allergen)| {
            map.entry(allergen)
                .and_modify(|ingredients| {
                    *ingredients = ingredients
                        .intersection(&food.ingredients)
                        .cloned()
                        .collect()
                })
                .or_insert_with(|| food.ingredients.clone());
            map
        });

    println!("{:?}", allergen_ingredients);

    HashMap::new()
}

#[test]
fn test_part_one() {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    let foods = parse_input(input);
    println!("{:?}", foods[3]);
}
