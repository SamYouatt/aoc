use std::collections::HashSet;

struct Card {
    winners: HashSet<usize>,
    havers: HashSet<usize>,
}

impl Card {
    fn parse(line: &str) -> Card {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winners, havers) = numbers.split_once('|').unwrap();

        fn parse_scores(scores: &str) -> HashSet<usize> {
            scores
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        }

        Card {
            winners: parse_scores(winners),
            havers: parse_scores(havers),
        }
    }

    fn count_winners(&self) -> usize {
        self.winners.intersection(&self.havers).count()
    }
}

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {}", answer1);
}

fn part_1(input: &str) -> usize {
    let cards: Vec<Card> = input.lines().map(|line| Card::parse(line)).collect();

    cards
        .iter()
        .map(|card| card.count_winners())
        .filter(|winners| *winners > 0)
        .fold(0, |total, winners| total + (2usize.pow(winners as u32 - 1)))
}
