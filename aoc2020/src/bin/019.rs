use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("../../inputs/019.txt");

    let rules: HashMap<usize, Rule> = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let (num, rule) = rule_parser::root(line).unwrap();
            (num, rule)
        })
        .collect();

    let strings: Vec<&str> = input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .collect();

    let start = Instant::now();
    println!(
        "Part one: {} in {:#?}",
        part_one(&rules, &strings),
        start.elapsed()
    );

    let start = Instant::now();
    println!(
        "Part two: {} in {:#?}",
        part_two(&rules, &strings),
        start.elapsed()
    );
}

fn part_one(rules: &HashMap<usize, Rule>, strings: &[&str]) -> usize {
    strings
        .iter()
        // matches recursively returns the number of letter in the string that were succesfully consumed
        // if this number is not the same as the strings length then it doesn't pass the grammar
        .filter(|string| {
            matches(rules, string.as_bytes(), 0)
                .map(|number| number == string.len())
                .unwrap_or(false)
        })
        .count()
}

// use the fact that 8 and 11 only appear in 0, and skip them and see if the string would work for those they call
// which is 42 then 31
fn part_two(rules: &HashMap<usize, Rule>, strings: &[&str]) -> usize {
    strings
        .iter()
        .filter(|string| works_for_42(string.as_bytes(), rules))
        .count()
}

// returns an option containing the number of letters in the string that were succesfully consumed
// by walking throught the rules
fn matches(rules: &HashMap<usize, Rule>, string: &[u8], rule_index: usize) -> Option<usize> {
    // get the current rule from the hash map
    match &rules.get(&rule_index) {
        // base case, all the string has been consumed
        Some(Rule::Literal(_)) if string.is_empty() => None,
        // if it is a literal test if it matches the first character in the remaining string
        // if it does match then return 1 which is added to the recursion result
        Some(Rule::Literal(c)) if &string[0] == c => Some(1),
        // the character didn't match so return none
        Some(Rule::Literal(_)) => None,
        // for each value in the sequence we will try and call the recursion
        // it is a try fold because some will return none
        // it is an accumulator that starts at 0, the beginning of the string it was passed
        // then every succesful match that increases the accumulator by 1, the string is consumed
        // slightly more by the recursion
        Some(Rule::Sequence(seq)) => seq.iter().try_fold(0, |c, rule| {
            matches(rules, &string[c..], *rule).map(|num| num + c)
        }),
        // same as sequence but does one first, if the try fold returns an error then try
        // the other sequence
        Some(Rule::SequenceOr(seq1, seq2)) => seq1
            .iter()
            .try_fold(0, |c, rule| {
                matches(rules, &string[c..], *rule).map(|num| num + c)
            })
            .or_else(|| {
                seq2.iter().try_fold(0, |c, rule| {
                    matches(rules, &string[c..], *rule).map(|num| num + c)
                })
            }),
        None => None,
    }
}

// a modified matches function which returns an option of the remaining string after consuming a character
fn matches_consumes<'a>(
    string: &'a [u8],
    rule: usize,
    rules: &HashMap<usize, Rule>,
) -> Option<&'a [u8]> {
    match &rules.get(&rule) {
        // base case, if the string is empty return none
        Some(Rule::Literal(_)) if string.is_empty() => None,
        // if the first character matches then consume it
        Some(Rule::Literal(c)) if &string[0] == c => Some(&string[1..]),
        // other base case, the first element in the string cannot be consumed, return none
        Some(Rule::Literal(_)) => None,
        // the string starts as what was received and over the course of the fold it gets updated
        // with the consumed values, until the try fold fails
        Some(Rule::Sequence(a)) => a.iter().try_fold(string, |string, &rule| {
            matches_consumes(string, rule, rules)
        }),
        // same as sequence but trying first sequence first then the second if the first fails
        Some(Rule::SequenceOr(a, b)) => a
            .iter()
            .try_fold(string, |string, &rule| {
                matches_consumes(string, rule, rules)
            })
            .or_else(|| {
                b.iter()
                    .try_fold(string, |m, &r| matches_consumes(m, r, rules))
            }),
        None => None,
    }
}

// starts at 42
fn works_for_42(string: &[u8], rules: &HashMap<usize, Rule>) -> bool {
    (0..)
        .try_fold(string, |string, depth| {
            match matches_consumes(string, 42, rules) {
                Some(string) if works_for_31(depth, string, rules) => Err(true),
                Some(string) => Ok(string),
                None => Err(false),
            }
        })
        .err()
        .unwrap()
}

// starts at 31 and sees if it gets consumed
fn works_for_31(limit: usize, string: &[u8], rules: &HashMap<usize, Rule>) -> bool {
    (0..limit)
        .try_fold(string, |string, _| {
            match matches_consumes(string, 31, rules) {
                Some(string) if string.is_empty() => Err(true),
                Some(string) => Ok(string),
                None => Err(false),
            }
        })
        .err()
        .unwrap_or(false)
}

#[derive(Debug, PartialEq)]
pub enum Rule {
    Literal(u8),
    Sequence(Vec<usize>),
    SequenceOr(Vec<usize>, Vec<usize>),
}

peg::parser! {
    grammar rule_parser() for str {
        pub rule root() -> (usize, Rule)
            = n:number() ":" _ r:rule_body() { (n, r) }

        rule rule_body() -> Rule
            = r:literal() / r:sequence_or() / r:sequence()  { r }

        rule literal() -> Rule
            = c:char() { Rule::Literal(c) }

        rule sequence() -> Rule
            = n:number() **<1,2> " " { Rule::Sequence(n) }

        rule sequence_or() -> Rule
            = x:number() **<1,2> " " _ "|" _ y:number() **<1,2> " " { Rule::SequenceOr(x, y)}

        rule char() -> u8
            = "\"" c:$("a" / "b") "\"" { c.as_bytes()[0] }

        rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule _()
            = " "*
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::{rule_parser, Rule};

    #[test_case("44: 91 71 | 77 109", 44, Rule::SequenceOr(vec![91, 71], vec![77, 109]) ; "Parse sequence or")]
    #[test_case("28: 77 | 91", 28, Rule::SequenceOr(vec![77], vec![91]) ; "Parse sequence or single value")]
    #[test_case("77: \"a\"", 77, Rule::Literal(b'a') ; "Parse literal")]
    #[test_case("39: 91 22", 39, Rule::Sequence(vec![91, 22]) ; "Parse sequence")]
    #[test_case("8: 42", 8, Rule::Sequence(vec![42]) ; "Parse sequence single value")]
    fn test_rule_parser(rule: &str, expected_num: usize, expected_rule: Rule) {
        let (num, rule) = rule_parser::root(rule).unwrap();
        assert_eq!(num, expected_num);
        assert_eq!(rule, expected_rule);
    }
}
