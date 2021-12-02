use std::time::Instant;

use itertools::Itertools;
use multimap::MultiMap;

fn main() {
    let input = include_str!("../../inputs/007.txt");
    let rules = parse_rules(input);

    let mut start = Instant::now();
    println!("Part one answer: {}", part_one(&rules));
    println!("Time taken: {:#?}", start.elapsed());

    start = Instant::now();
    println!("Part two answer: {}", part_two(&rules));
    println!("Time taken: {:#?}", start.elapsed());
}

fn part_one(rules: &Rules) -> usize {
    let reverse_rules = reverse_graph(&rules);

    let goal = ("shiny", "gold");
    walk(&reverse_rules, &goal).unique().count()
}

fn part_two(rules: &Rules) -> usize {
    let goal = ("shiny", "gold");
    walk_and_count(&rules, &goal).sum()
}

type BagDescription<'a> = (&'a str, &'a str);

type Rules<'a> = MultiMap<BagDescription<'a>, (usize, BagDescription<'a>)>;

fn parse_rules(input: &str) -> Rules<'_> {
    let mut rules: Rules = Rules::default();

    peg::parser! {
        pub (crate) grammar parser() for str {
            pub (crate) rule root(r: &mut Rules<'input>)
                = (line(r) "." whitespace()*)* ![_]

            rule line(r: &mut Rules<'input>)
                = description:bag_description() " contain " rules:rules() {
                    if let Some(rules) = rules {
                        rules.iter().for_each(|rule| r.insert(description, *rule))
                    }
                }

            rule bag_description() -> BagDescription<'input>
                = adjective:word() " " color:word() " bag" "s"? { (adjective, color) }

            rule rules() -> Option<Vec<(usize, BagDescription<'input>)>>
                = rules:ruleA()+ { Some(rules) } / "no other bags" { None }

            rule ruleA() -> (usize, BagDescription<'input>)
                = r:ruleX() ", "? { r }

            rule ruleX() -> (usize, BagDescription<'input>)
                = quantity:number() " " description:bag_description() { (quantity, description) }

            rule number() -> usize
                = e:$(['0'..='9']+) { e.parse().unwrap() }

            rule word() -> &'input str
                = $((!whitespace()[_])*)

            rule whitespace()
                = [' ' | '\t' | '\r' | '\n']
        }
    }

    parser::root(input, &mut rules).unwrap();
    rules
}

fn reverse_graph<'a>(graph: &Rules<'a>) -> Rules<'a> {
    graph
        .iter_all()
        .map(|(&node, neighbours)| {
            neighbours
                .iter()
                .map(move |&(quantity, neighbour)| (neighbour, (quantity, node)))
        })
        .flatten()
        .collect()
}

fn walk<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    root: &(&'iter str, &'iter str),
) -> Box<dyn Iterator<Item = (&'elems str, &'elems str)> + 'iter> {
    Box::new(
        graph
            .get_vec(root)
            .into_iter()
            .flatten()
            .map(move |&(_, neighbour)| std::iter::once(neighbour).chain(walk(graph, &neighbour)))
            .flatten(),
    )
}

fn walk_and_count<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    root: &(&'iter str, &'iter str),
) -> Box<dyn Iterator<Item = usize> + 'iter> {
    Box::new(
        graph
            .get_vec(root)
            .into_iter()
            .flatten()
            .map(move |&(quantity, contents)| {
                std::iter::once(quantity)
                    .chain(walk_and_count(graph, &contents).map(move |x| x * quantity))
            })
            .flatten(),
    )
}
