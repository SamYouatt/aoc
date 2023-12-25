use std::collections::HashSet;

use hashbrown::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
}

fn part_1(input: &str) -> usize {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut edges = HashSet::new();

    for line in input.lines() {
        let (source, rest) = line.split_once(": ").unwrap();
        let dests = rest.split_whitespace();

        for dest in dests {
            graph.entry(source).or_insert(HashSet::new()).insert(dest);
            graph.entry(dest).or_insert(HashSet::new()).insert(source);
            edges.insert((source, dest));
        }
    }

    let dot_format = get_dot_format(&edges);

    std::fs::write("graph.dot", dot_format).unwrap();

    // from my graph I can see that my linking nodes are crg - krf, jet - rgv, fmr - zhg
    let to_remove = [("crg", "krf"), ("jct", "rgv"), ("fmr", "zhg")];

    for pair in to_remove {
        graph.get_mut(pair.0).unwrap().remove(pair.1);
        graph.get_mut(pair.1).unwrap().remove(pair.0);
    }

    let start = graph.iter().next().unwrap().0;

    let mut visited = HashSet::new();
    let mut stack = vec![start];

    while let Some(next) = stack.pop() {
        if visited.contains(next) {
            continue;
        }

        visited.insert(next);
        stack.extend(&graph[next]);
    }

    let cluster_a_size = visited.len();
    let total_graph = graph.len();

    cluster_a_size * (total_graph - cluster_a_size)
}

fn get_dot_format(edges: &HashSet<(&str, &str)>) -> String {
    let mut output = String::from("graph {\n");

    for edge in edges {
        output += &format!("{} -- {}\n", edge.0, edge.1);
    }

    output += "}";

    output
}
