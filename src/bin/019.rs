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
        part_one(&rules, strings),
        start.elapsed()
    );
}

fn part_one(rules: &HashMap<usize, Rule>, strings: Vec<&str>) -> usize {
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

// returns an option containing the number of letters in the string that were succesfully consumed
// by walking throught the rules
fn matches(rules: &HashMap<usize, Rule>, string: &[u8], rule_index: usize) -> Option<usize> {
    // base case, all the string has been consumed
    if string.is_empty() {
        return None;
    }

    // get the current rule from the hash map
    match &rules.get(&rule_index) {
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
