use multimap::MultiMap;

fn main() {
    let input = include_str!("../../inputs/007.txt");

    let rules = parse_rules(input);
    dbg!(rules);
}

type BagDescription<'a> = (&'a str, &'a str);

type Rules<'a> = MultiMap<BagDescription<'a>, (usize, BagDescription<'a>)>;

fn parse_rules(input: &str) -> Rules<'_> {
    let mut rules: Rules = Rules::default();

    peg::parser! {
        grammar parser() for str {
            pub (crate) rule root(r: &mut Rules<'input>)
                = (line(r) "." whitespace()*)* ![_]

            rule line(r: &mut Rules<'input>)
                = description:bag_description() " contain" rules:rules() {
                    if let Some(rules) = rules {
                        rules.iter().for_each(|rule| r.insert(description, *rule))
                    }
                }

            rule bag_description() -> BagDescription<'input>
                = adjective:word() " " color:word() " bag" "s"? { (adjective, color) }

            rule rules() -> Option<Vec<(usize, BagDescription<'input>)>>
                = rules:ruleA()+ { Some(rules) } / { None }

            rule ruleA() -> (usize, BagDescription<'input>)
                = r:ruleX() ", "? { r }

            rule ruleX() -> (usize, BagDescription<'input>)
                = quantity:number() " " description:bag_description() { (quantity, description) }

            rule word() -> &'input str
                = $((!whitespace()[_])*)

            rule number() -> usize
                = quantity:$(['0'..='9']+) { quantity.parse().unwrap() }

            rule whitespace()
                = [' ' | '\r' | '\n']
        }
    }

    parser::root(input, &mut rules).unwrap();
    rules
}
