use std::collections::{hash_map::Entry, HashMap, VecDeque};

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
}

#[derive(Debug)]
enum Module<'a> {
    Broadcast(Vec<&'a str>),
    FlipFlop(bool, Vec<&'a str>),
    Conjuction(HashMap<&'a str, bool>, Vec<&'a str>),
}

fn part_1(input: &str) -> usize {
    let mut modules = parse_modules(input);

    let mut low_signals = 0;
    let mut high_signals = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::from_iter([("button", "broadcaster", false)]);

        while let Some((prev, current, signal)) = queue.pop_front() {
            if signal {
                high_signals += 1
            } else {
                low_signals += 1
            }

            if let Entry::Occupied(mut current_module) = modules.entry(current) {
                let current_module = current_module.get_mut();

                match current_module {
                    Module::Broadcast(destinations) => {
                        for destination in destinations {
                            queue.push_back((current, destination, signal));
                        }
                    }
                    Module::FlipFlop(ref mut ff_state, destinations) => {
                        match (signal, &ff_state) {
                            // turns off and sends low pulse
                            (false, true) => {
                                *ff_state = false;
                                for destination in destinations {
                                    queue.push_back((current, destination, false));
                                }
                            }
                            // turns on and sends high pulse
                            (false, false) => {
                                *ff_state = true;
                                for destination in destinations {
                                    queue.push_back((current, destination, true));
                                }
                            }
                            // high pulse does nothing
                            _ => {}
                        }
                    }
                    Module::Conjuction(ref mut inputs, destinations) => {
                        // update its memory for the received signal
                        inputs.insert(prev, signal);

                        let signal_to_send = !inputs.values().all(|&sig| sig);

                        for destination in destinations {
                            queue.push_back((current, destination, signal_to_send));
                        }
                    }
                }
            }
        }
    }

    low_signals * high_signals
}

fn parse_modules(input: &str) -> HashMap<&str, Module> {
    let mut modules: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (module, rest) = line.split_once(" -> ").unwrap();
            let destinations: Vec<_> = rest.split(", ").collect();

            if module == "broadcaster" {
                (module, Module::Broadcast(destinations))
            } else if let Some(name) = module.strip_prefix('%') {
                (name, Module::FlipFlop(false, destinations))
            } else if let Some(name) = module.strip_prefix('&') {
                (name, Module::Conjuction(HashMap::new(), destinations))
            } else {
                panic!("Unknown module");
            }
        })
        .collect();

    let all_connections: Vec<_> = modules
        .iter()
        .flat_map(|(&source, module)| {
            let destinations = match module {
                Module::Broadcast(destinations) => destinations,
                Module::FlipFlop(_, destinations) => destinations,
                Module::Conjuction(_, destinations) => destinations,
            };

            destinations
                .iter()
                .map(|&dest| (source, dest))
                .collect::<Vec<(&str, &str)>>()
        })
        .collect();

    // build up the conjugate backlinks from the list of all connections
    for (source, target) in all_connections {
        if let Entry::Occupied(mut target_module) = modules.entry(target) {
            if let Module::Conjuction(ref mut conj_inputs, _) = target_module.get_mut() {
                conj_inputs.insert(source, false);
            }
        }
    }

    modules
}

#[test]
fn part_1_example_1() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    let answer = part_1(input);

    assert_eq!(answer, 32000000);
}

#[test]
fn part_2_example_2() {
    let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    let answer = part_1(input);

    assert_eq!(answer, 11687500);
}
