use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[derive(Debug)]
enum Type {
    AND,
    OR,
    XOR,
    NONE,
}

#[derive(Debug)]
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

fn part_2(input: &str) -> usize {
    todo!()
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
