use std::{fs, vec};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::convert::identity;

use itertools::Itertools;
use ModuleType::{Button, Conjunction, FlipFlop, Output};
use crate::common::{Direction, next_coord};
use crate::common::Direction::{East, North, South, West};
use crate::day20::ModuleType::Broadcaster;

pub fn main() {
    // let a = Module { label: "button".to_string(), module_type: Button, connected_to: vec!["broadcaster".to_string()], state: false };
    // let b = Module { label: "broadcaster".to_string(), module_type: Button, connected_to: vec![], state: false };
    // let mut map: HashMap<String, Box<Module>> = HashMap::new();
    // map.insert("button".to_string(), Box::new(a));
    // map.insert("broadcaster".to_string(), Box::new(b));
    //
    // map.entry("button".to_string()).and_modify(|a| mutate(a.as_mut(), &mut map));

    let input = &fs::read_to_string("./inputs/day20/input.txt").unwrap();
    part1(input);
// part2(input);
}

// fn mutate(a: &mut Module, map: &mut HashMap<String, Module>){
//     a.connected_to.iter().for_each(|to| {
//         let target = map.get_mut(to).unwrap();
//         mutate(target, map)
//     });
// }


fn part1(input: &str) -> usize {
    // let instructions = parse_pt1(input);
    // // Take the naive flood-fill approach
    // let dug_squares = count_interior_squares(&instructions);
    // println!("Part 1: dug {dug_squares} squares");
    // dug_squares
    let (mut connections, mut modules) = parse(input);
    let mut counter = (0, 0);
    for _ in 0..1000 {
        send_signal(&"".to_string(), &"button".to_string(), false, &mut connections, &mut modules, (&mut counter.0, &mut counter.1));
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
        (label.clone(), Module { label, module_type, state: false, memory: HashMap::new()})
    }).collect::<HashMap<String, Module>>();
    let mut connections_map = input.lines().map(|line| {
        let (a, b) = line.split(" -> ").tuple_windows().next().unwrap();
        let label = if a == "broadcaster" { a.to_string() } else { a[1..].to_string() };
        let connected_to_string = b.split(", ").map(str::to_string).collect::<Vec<_>>();
        (label.clone(), connected_to_string)
    }).collect::<HashMap<String, Vec<String>>>();

    module_map.insert("button".to_string(), Module { label: "button".to_string(), module_type: Button, state: false, memory: HashMap::new() });
    connections_map.insert("button".to_string(), vec!["broadcaster".to_string()]);
    (connections_map, module_map)
}

fn send_signal(from_name: &String, target_name: &String, high_signal: bool, map: &mut HashMap<String, Vec<String>>, modules: &mut HashMap<String, Module>, (low, high):  (&mut usize, &mut usize)) {
    if !from_name.is_empty() {
        if high_signal {
            *high += 1
        } else {
            *low += 1
        }

        let string = if high_signal { "high" } else {"low"};
        println!("{from_name} -{string}-> {target_name}");
    }

    let target = modules.get_mut(target_name).unwrap();
    // print!("{:?}: ", target.module_type);
    match target.module_type {
        Output => {
            // println!("Received {high_signal} signal from {from_name} to output {:?}", target_name)
        }
        Button => {
            // println!("Sending {high_signal} signal to broadcaster");
            for next_node in &map.clone()[target_name] {
                send_signal(target_name, next_node, high_signal, map, modules, (low, high))
            }
        }
        Broadcaster => {
            // println!("Sending {high_signal} signal to all connectors {:?}", map[target_name]);
            for next_node in &map.clone()[target_name] {
                send_signal(target_name, next_node, high_signal, map, modules, (low, high))
            }
        }
        FlipFlop => {
            // println!("Received {high_signal} signal from '{from_name}' at flip-flop '{}'", target_name);
            if high_signal {
                // ignore
            } else {
                // flip
                let new_state = !target.state;
                target.state = new_state;

                for next_node in &map.clone()[target_name] {
                    send_signal(target_name, next_node, new_state, map, modules, (low, high))
                }
            }
        }
        Conjunction => {
            // println!("Received {high_signal} signal from '{from_name}' at conjunction '{target_name}'");
            let memory_entry = target.memory.entry(from_name.clone()).or_insert(high_signal);
            *memory_entry = high_signal;
            // if all high send low, else send high
            let signal_to_send =  !target.memory.values().all(|&b| b); // memory is guaranteed non-empty
            for next_node in &map.clone()[target_name] {
                send_signal(target_name, next_node, signal_to_send, map, modules, (low, high))
            }
        }
    }
}


#[cfg(test)]
mod tests {
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
}