use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/018.txt");

    let start = Instant::now();
    println!("Part one: {} in {:#?}", part_one(input), start.elapsed());

    let start = Instant::now();
    println!("Part two: {} in {:#?}", part_two(input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| part_one_parser::expression(line).unwrap())
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| part_two_parser::expression(line).unwrap())
        .sum()
}

peg::parser! {
    grammar part_one_parser() for str {
        pub rule expression() -> usize
            // the precedence macro is used to define precedence levels
            // -- indicates a different level of precedence
            // lower on the list is higher precedence so this grammar parses bracketed expressions and numbers as highest
            // this means they are read left to write
            = precedence! {
                x:(@) " + " y:@ { x + y }
                x:(@) " * " y:@ { x * y }
                --
                "(" e:expression() ")" { e }
                n:number() { n }
            }

        rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }
    }
}

peg::parser! {
    grammar part_two_parser() for str {
        pub rule expression() -> usize
            // same as the one above but with an additional precidence level to account for
            // addition being considered before multiplication
            = precedence! {
                x:(@) " * " y:@ { x * y }
                --
                x:(@) " + " y:@ { x + y }
                --
                "(" e:expression() ")" { e }
                n:number() { n }
            }

        rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::{part_one_parser, part_two_parser};

    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51 ; "first example")]
    #[test_case("2 * 3 + (4 * 5)", 26 ; "second example")]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437 ; "third example")]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240 ; "fourth example")]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632 ; "fifth example")]
    fn test_parser_one(expression: &str, expected: usize) {
        assert_eq!(part_one_parser::expression(expression).unwrap(), expected);
    }

    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51 ; "first example")]
    #[test_case("2 * 3 + (4 * 5)", 46 ; "second example")]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445 ; "third example")]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060 ; "fourth example")]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340 ; "fifth example")]
    fn test_parser_two(expression: &str, expected: usize) {
        assert_eq!(part_two_parser::expression(expression).unwrap(), expected);
    }
}
