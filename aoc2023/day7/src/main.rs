use std::{char, collections::HashMap};

#[derive(Debug)]
struct Hand {
    cards: Vec<usize>,
    score: usize,
}

impl Hand {
    fn parse(line: &str) -> Hand {
        let cards = line.chars().map(|card| Hand::card_to_score(card)).collect();

        let mut instances: HashMap<usize, usize> = HashMap::new();
        for &card in &cards {
            let count = instances.entry(card).or_insert(0);
            *count += 1;
        }

        let mut instances_vec: Vec<_> = instances.iter().collect();
        instances_vec.sort_by(|a, b| b.1.cmp(&a.1));

        let score = 3 * instances_vec[0].1 + instances_vec.get(1).map(|x| x.1).unwrap_or(&0);

        Hand { cards, score }
    }

    fn compare_score(&self, other: &Hand) -> std::cmp::Ordering {
        let base_cmp = self.score.cmp(&other.score);

        if base_cmp != std::cmp::Ordering::Equal {
            return base_cmp;
        }

        let mut inspecting = 0;
        while inspecting < 5 {
            let base_cmp = self.cards[inspecting].cmp(&other.cards[inspecting]);
            if base_cmp != std::cmp::Ordering::Equal {
                return base_cmp;
            }
            inspecting += 1;
        }

        std::cmp::Ordering::Equal
    }

    fn card_to_score(card: char) -> usize {
        match card {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => panic!("unknown card"),
        }
    }
}

#[derive(Debug)]
struct Game {
    hand: Hand,
    bid: usize,
}

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
}

fn part_1(input: &str) -> usize {
    let mut games: Vec<Game> = input
        .lines()
        .map(|game| {
            let (hand, bid) = game.split_once(' ').unwrap();

            Game {
                hand: Hand::parse(hand),
                bid: bid.parse::<usize>().unwrap(),
            }
        })
        .collect();

    games.sort_by(|a, b| a.hand.compare_score(&b.hand));

    games
        .iter()
        .enumerate()
        .map(|(i, game)| game.bid * (i + 1))
        .sum()
}
