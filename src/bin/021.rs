fn main() {
    let input = include_str!("../../inputs/021.txt");
    let foods = parse_input(input);

    println!("{:?}", foods[0]);
}

fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            let (ingredients, allergens) = line.split_once(" (contains ").unwrap();
            (
                ingredients.split(' ').collect(),
                allergens.trim_end_matches(')').split(", ").collect(),
            )
        })
        .collect()
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
