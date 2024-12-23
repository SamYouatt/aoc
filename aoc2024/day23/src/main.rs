use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (x, y) = line.split_once('-').unwrap();

        graph.entry(x).or_default().insert(y);
        graph.entry(y).or_default().insert(x);
    }

    let mut triangles = HashSet::new();
    for (node, conns) in graph.iter() {
        if !node.starts_with('t') {
            continue;
        }

        for (n1, n2) in conns.iter().tuple_combinations() {
            if graph
                .get(n1)
                .map_or(false, |n1_conns| n1_conns.contains(n2))
                && graph
                    .get(n2)
                    .map_or(false, |n2_conns| n2_conns.contains(n1))
            {
                let mut triangle = vec![node, n1, n2];
                triangle.sort();

                triangles.insert(triangle);
            }
        }
    }

    triangles.len()
}
