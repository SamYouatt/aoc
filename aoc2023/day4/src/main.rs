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
    let cards: Vec<Card> = input.lines().map(|line| Card::parse(line)).collect();

    let answer1 = part_1(&cards);
    println!("Part 1: {}", answer1);

    let answer2 = part_2(&cards);
    println!("Part 2: {}", answer2);
}

fn part_1(cards: &Vec<Card>) -> usize {
    cards
        .iter()
        .map(|card| card.count_winners())
        .filter(|winners| *winners > 0)
        .fold(0, |total, winners| total + (2usize.pow(winners as u32 - 1)))
}

fn part_2(cards: &Vec<Card>) -> usize {
    let card_wins: Vec<usize> = cards.iter().map(|card| card.count_winners()).collect();

    let mut card_copies = vec![1; cards.len()];

    for (card_index, wins) in card_wins.iter().enumerate() {
        for card_offset in 1..=*wins {
            let card_copy_won = card_index + card_offset;
            let num_copies = card_copies[card_index];
            card_copies[card_copy_won] += num_copies
        }
    }

    card_copies.iter().sum()
}
