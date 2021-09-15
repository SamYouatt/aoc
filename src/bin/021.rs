use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

fn main() {
    let input = include_str!("../../inputs/021.txt");
    let foods = parse_input(input);

    let mapping = get_allergic_ingredients(&foods);
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

fn get_allergic_ingredients<'a>(foods: &[Food<'a>]) -> HashMap<&'a str, &'a str> {
    // first find the list of an allergen and all the ingerdients it could be in
    // to do this go through the list and each time an allergen appears, check the list of ingredients
    // for that allergen
    // if none exist then put the whole list of ingredients in
    // once it is filled subsequent ones should update the value by findining the intersection of the current ingredients
    // and the new possible ones in the list
    let mut allergen_ingredients: HashMap<&str, HashSet<&str>> = foods
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

    let mut allergen_mappings: HashMap<&str, &str> = HashMap::new();
    while !allergen_ingredients.is_empty() {
        let to_remove: Option<(&str, &str)>;

        println!("remaining allergens: {:?}", allergen_ingredients);
        // allergen_ingredients.iter().for_each(|allergen| {
        //     if allergen.1.len() == 1 {
        //         println!("allergen: {:?}", allergen);
        //         let ingredient = Vec::from_iter(allergen.1)[0];
        //         to_remove = Some((allergen.0, ingredient));
        //         allergen_mappings.insert(allergen.0, *ingredient);
        //     }
        // });

        let allergen = allergen_ingredients
            .iter()
            .find(|allergen| allergen.1.len() == 1)
            .unwrap();
        println!("allergen: {:?}\n", allergen);
        let ingredient = Vec::from_iter(allergen.1)[0];
        to_remove = Some((allergen.0, ingredient));
        allergen_mappings.insert(allergen.0, *ingredient);

        if let Some(to_remove) = to_remove {
            allergen_ingredients.remove(to_remove.0);
            for allergen in &mut allergen_ingredients {
                if allergen.1.contains(to_remove.1) {
                    allergen.1.remove(to_remove.1);
                }
            }
        }
    }

    allergen_mappings
}

#[test]
fn test_part_one() {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    let foods = parse_input(input);
    get_allergic_ingredients(&foods);
}
