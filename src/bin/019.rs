fn main() {
    let input = include_str!("../../inputs/019.txt");

    let test = "44: 91 71 | 77 109";

    let (num, rule) = rule_parser::root(test).unwrap();

    println!("{} {:#?}", num, rule);
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
