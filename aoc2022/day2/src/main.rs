fn main() {
    let input = include_str!("input.txt");

    let part1_answer: usize = input
        .lines()
        .map(|round| {
            let plays: Vec<&str> = round.split(" ").collect();
            play_round(plays[0], plays[1])
        })
        .sum();

    println!("Part 1 answer: {part1_answer}");

    let part2_answer: usize = input
        .lines()
        .map(|round| {
            let sections: Vec<&str> = round.split(" ").collect();
            play_round_with_strategy(sections[0], sections[1])
        })
        .sum();

    println!("Part 2 answer: {part2_answer}");
}

fn play_round(opponent: &str, player: &str) -> usize {
    let choice_score = match player {
        "X" => 1,
        "Y" => 2,
        _ => 3,
    };

    let round_score = match (opponent, player) {
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        _ => 0,
    };

    choice_score + round_score
}

fn play_round_with_strategy(opponent: &str, required: &str) -> usize {
    // X - Lose, Y - Draw, Z - Win
    let player_choice = match (opponent, required) {
        ("A", "Y") | ("B", "X") | ("C", "Z") => "X",
        ("B", "Y") | ("C", "X") | ("A", "Z") => "Y",
        _ => "Z",
    };

    play_round(opponent, player_choice)
}
