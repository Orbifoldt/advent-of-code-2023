use std::{fs, vec};
use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use ModuleType::{Broadcaster, Button, Conjunction, FlipFlop, Output};

pub fn main() {
    let input = &fs::read_to_string("./inputs/day20/input.txt").unwrap();
    part1(input);
// part2(input);
}

fn part1(input: &str) -> usize {
    let (mut connections, mut modules) = parse(input);
    let mut counter = (0, 0);
    for _ in 0..1000 {
        send_signal_bfs(&mut connections, &mut modules, (&mut counter.0, &mut counter.1));
    }
    let product = counter.0*counter.1;
    println!("Total sent {} low and {} high signals, product is {}", counter.0, counter.1, product);
    product
}

fn part2(input: &str) -> usize {
    todo!()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Module {
    label: String,
    module_type: ModuleType,
    state: bool,
    memory: HashMap<String, bool>
}

impl Module {
    pub fn new(label: String, module_type: ModuleType) -> Self {
        Self { label, module_type, state: false, memory: HashMap::new() }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum ModuleType {
    Broadcaster,
    Output,
    Button,
    FlipFlop,
    Conjunction,
}

fn parse(input: &str) -> (HashMap<String, Vec<String>>, HashMap<String, Module>) {
    let mut module_map = input.lines().map(|line| {
        let (a, b) = line.split(" -> ").tuple_windows().next().unwrap();
        let module_type = match a.chars().next().unwrap() {
            '%' => FlipFlop,
            '&' => Conjunction,
            _ => Broadcaster
        };
        let label = if module_type == Broadcaster { a.to_string() } else { a[1..].to_string() };
        (label.clone(), Module::new(label, module_type))
    }).collect::<HashMap<String, Module>>();

    let mut connections_map = input.lines().map(|line| {
        let (a, b) = line.split(" -> ").tuple_windows().next().unwrap();
        let label = if a == "broadcaster" { a.to_string() } else { a[1..].to_string() };
        let connected_to_string = b.split(", ").map(str::to_string).collect::<Vec<_>>();
        (label.clone(), connected_to_string)
    }).collect::<HashMap<String, Vec<String>>>();

    module_map.insert("button".to_string(), Module::new("button".to_string(),  Button));
    connections_map.insert("button".to_string(), vec!["broadcaster".to_string()]);

    // Conjunctions need to know who are connected to them
    module_map.iter_mut().filter(|(label, module)| module.module_type == Conjunction)
        .for_each(|(label, conjunction)| {
            conjunction.memory = connections_map.iter().filter(|&(_, connections)| connections.contains(label))
                .map(|(other_label, _)| (other_label.clone(), false))
                .collect::<HashMap<String, bool>>();
        });

    (connections_map, module_map)
}

fn send_signal_bfs(map: &HashMap<String, Vec<String>>, modules: &mut HashMap<String, Module>, (low, high):  (&mut usize, &mut usize)) {
    let button = &"button".to_string();
    let broadcast = &"broadcaster".to_string();
    let mut signals_to_send = VecDeque::from([(button, broadcast, false)]);

    while let Some((from_name, current_name, high_signal)) = signals_to_send.pop_front() {
        if high_signal {
            *high += 1
        } else {
            *low += 1
        }
        let string = if high_signal { "high" } else {"low"};
        // println!("{from_name} -{string}-> {current_name}");

        let signal_to_send;
        if let Some(current) = modules.get_mut(current_name) {
            // print!("  > {current_name} ({:?}): ", current.module_type);
            match current.module_type {
                Button => unreachable!(),
                Broadcaster => {
                    // println!("Sending {high_signal} signal to all connectors {:?}", map[current_name]);
                    signal_to_send = high_signal;
                }
                FlipFlop => {
                    // println!("Received {high_signal} signal from '{from_name}'");
                    if !high_signal {
                        // flip
                        let new_state = !current.state;
                        current.state = new_state;
                        signal_to_send = new_state;
                    } else {
                        continue;
                    }
                }
                Conjunction => {
                    // println!("Received {high_signal} signal from '{from_name}'");
                    // println!("    Memory contains:");
                    // current.memory.iter().for_each(|(k,v)| println!("      {k}: {v}"));
                    let memory_entry = current.memory.entry(from_name.clone()).or_insert(high_signal);
                    *memory_entry = high_signal;
                    // println!("      *{from_name}: {high_signal}*");
                    // if all high send low, else send high
                    signal_to_send = !current.memory.values().all(|&b| b); // memory is guaranteed non-empty
                }
                Output => {
                    // println!("Received {high_signal} signal from {from_name}");
                    continue
                }
            }
            for next_node in &map[current_name] {
                signals_to_send.push_back((current_name, next_node, signal_to_send));
                // println!("    - Next turn sending from {current_name} signal {signal_to_send} to {next_node}");
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day20::part1;

    #[test]
    fn part_1_example_1() {
        let input = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(part1(input), 32000000)
    }

    #[test]
    fn part_1_example_2() {
        let input = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(part1(input), 11687500)
    }

    #[test]
    fn part_1_input() {
        let input = &fs::read_to_string("./inputs/day20/input.txt").unwrap();
        assert_eq!(part1(input), 666795063)
    }

}