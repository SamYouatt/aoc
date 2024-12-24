use std::collections::{HashMap, VecDeque};

use graphviz_rust::{
    attributes::{color_name, shape, NodeAttributes},
    dot_structures::{Edge, EdgeTy, Graph, Id, Node, NodeId, Stmt, Vertex},
    printer::{DotPrinter, PrinterContext},
};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[derive(Debug, Clone)]
enum Type {
    AND,
    OR,
    XOR,
    NONE,
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    a: Option<&'a str>,
    b: Option<&'a str>,
    gate_type: Type,
    value: Option<bool>,
}

impl Default for Gate<'_> {
    fn default() -> Self {
        Self {
            a: None,
            b: None,
            gate_type: Type::NONE,
            value: None,
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut circuit = parse_circuit(input);

    run_circuit(&mut circuit);

    let mut zs = circuit
        .keys()
        .filter(|k| k.starts_with('z'))
        .collect::<Vec<_>>();
    zs.sort();
    let bits = zs.iter().rev().fold(String::new(), |mut b, z| {
        let bit = match circuit[**z].value {
            Some(true) => '1',
            Some(false) => '0',
            None => panic!(),
        };
        b.push(bit);
        b
    });

    usize::from_str_radix(&bits, 2).unwrap()
}

// Observations
// - Every x and y input should go to an AND and an XOR
// - Other than first and last z bits, they should all be XOR
// - Sibling of z bit should be AND which is fed by OR and XOR
//
// Possibly wrong:
// z10, gpr
// krs, cpm
// ghp, z33
// z21, nks
//
// Note to future me:
// This runs to create two graphviz dot files, unchanged.dot and modified.dot
// Each node has its name and type, z nodes are coloured red to spot easily, input nodes are
// blue
// The modified graph will colour the changed pairs green so they can be double checked
fn part_2(input: &str) -> String {
    let original_circuit = parse_circuit(input);
    let mut modified = original_circuit.clone();

    output_graph(original_circuit, "unchanged.dot", &vec![]);

    swap_values(&mut modified, "z10", "gpr");
    swap_values(&mut modified, "krs", "cpm");
    swap_values(&mut modified, "ghp", "z33");
    swap_values(&mut modified, "z21", "nks");
    let mut modifications = vec!["z10", "gpr", "krs", "cpm", "ghp", "z33", "z21", "nks"];

    output_graph(modified, "modified.dot", &modifications);

    modifications.sort();
    modifications.join(",")
}

fn swap_values<'a>(map: &mut HashMap<&'a str, Gate>, a: &'a str, b: &'a str) {
    let v1 = map.get_mut(&a).and_then(|val| Some(std::mem::take(val)));
    let v2 = map.get_mut(&b).and_then(|val| Some(std::mem::take(val)));

    if let (Some(v1), Some(v2)) = (v1, v2) {
        map.insert(a, v2);
        map.insert(b, v1);
    }
}

fn output_graph(circuit: HashMap<&str, Gate<'_>>, filename: &str, modified: &Vec<&str>) {
    let mut graph = Graph::DiGraph {
        id: Id::Plain("Day24Graph".into()),
        strict: false,
        stmts: vec![],
    };

    for (n, node) in circuit.iter() {
        let node_type = String::from(match node.gate_type {
            Type::AND => "AND",
            Type::OR => "OR",
            Type::XOR => "XOR",
            Type::NONE => "in",
        });

        let label = format!(r#""{}: {}""#, n.to_string(), node_type);
        let mut attributes = vec![
            NodeAttributes::shape(shape::circle),
            NodeAttributes::label(label),
        ];

        if n.starts_with('z') {
            attributes.push(NodeAttributes::color(color_name::coral1));
            attributes.push(NodeAttributes::style("filled".to_string()));
        }

        if n.starts_with('x') || n.starts_with('y') {
            attributes.push(NodeAttributes::color(color_name::cornflowerblue));
            attributes.push(NodeAttributes::style("filled".to_string()));
        }

        if modified.contains(n) {
            attributes.push(NodeAttributes::color(color_name::darkseagreen1));
            attributes.push(NodeAttributes::style("filled".to_string()));
        }

        let stmt = Stmt::Node(Node {
            id: NodeId(Id::Plain(n.to_string()), None),
            attributes,
        });
        graph.add_stmt(stmt);
    }

    for (n, node) in circuit.iter() {
        let to = NodeId(Id::Plain(n.to_string()), None);

        if node.a.is_some() {
            let from = NodeId(Id::Plain(node.a.unwrap().to_string()), None);
            let edge = Edge {
                ty: EdgeTy::Pair(Vertex::N(from), Vertex::N(to.clone())),
                attributes: vec![],
            };
            let stmt = Stmt::Edge(edge);
            graph.add_stmt(stmt);
        }

        if node.b.is_some() {
            let from = NodeId(Id::Plain(node.b.unwrap().to_string()), None);
            let edge = Edge {
                ty: EdgeTy::Pair(Vertex::N(from), Vertex::N(to)),
                attributes: vec![],
            };
            let stmt = Stmt::Edge(edge);
            graph.add_stmt(stmt);
        }
    }

    let dot_output = graph.print(&mut PrinterContext::default());

    std::fs::write(filename, dot_output).unwrap();
}

fn run_circuit(circuit: &mut HashMap<&str, Gate<'_>>) {
    let mut queue = VecDeque::new();
    circuit.keys().for_each(|k| queue.push_back(*k));

    while let Some(gate_key) = queue.pop_front() {
        let gate = circuit.get(gate_key).unwrap();
        if gate.value.is_some() {
            continue;
        }

        let dep_a = circuit.get(gate.a.unwrap()).unwrap();
        let dep_b = &circuit[gate.b.unwrap()];

        if dep_a.value.is_none() || dep_b.value.is_none() {
            queue.push_back(gate_key);
            continue;
        }

        let val_a = dep_a.value.unwrap();
        let val_b = dep_b.value.unwrap();

        let gate = circuit.get_mut(gate_key).unwrap();
        gate.value = gate_logic(gate, val_a, val_b);
    }
}

fn gate_logic(gate: &Gate<'_>, val_a: bool, val_b: bool) -> Option<bool> {
    match (&gate.gate_type, val_a, val_b) {
        (Type::AND, a, b) => Some(a & b),
        (Type::OR, a, b) => Some(a | b),
        (Type::XOR, a, b) => Some(a ^ b),
        (Type::NONE, _, _) => None,
    }
}

fn parse_circuit(input: &str) -> HashMap<&str, Gate<'_>> {
    let (wires, raw_circuit) = input.split_once("\n\n").unwrap();

    let mut circuit = HashMap::new();
    for chunk in raw_circuit.lines() {
        let (a, rest) = chunk.split_once(' ').unwrap();
        let (gate_type, rest) = rest.split_once(' ').unwrap();
        let (b, rest) = rest.split_once(' ').unwrap();
        let (_, gate_name) = rest.split_once(' ').unwrap();

        circuit.entry(a).or_default();
        circuit.entry(b).or_default();

        let gate = Gate {
            a: Some(a),
            b: Some(b),
            gate_type: match gate_type {
                "AND" => Type::AND,
                "OR" => Type::OR,
                "XOR" => Type::XOR,
                _ => panic!(),
            },
            value: None,
        };

        *circuit.entry(gate_name).or_default() = gate;
    }

    for wire in wires.lines() {
        let (gate, signal) = wire.split_once(": ").unwrap();
        let signal = match signal {
            "0" => false,
            "1" => true,
            _ => panic!(),
        };

        circuit.get_mut(gate).unwrap().value = Some(signal);
    }

    circuit
}
