use std::{fs, vec};
use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use ModuleType::{Broadcaster, Button, Conjunction, FlipFlop, Output};
use crate::common::lcm;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day20/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> usize {
    let (mut connections, mut modules) = parse(input);
    let (low, high) = (0..1000)
        .map(|_| send_signal_bfs(&mut connections, &mut modules))
        .reduce(|(sum_low, sum_high), (low, high)| (sum_low + low, sum_high + high)).unwrap();
    let product = low * high;
    println!("Part 1: In total sent {low} low and {high} high signals, product is {product}");
    product
}

// Render the graph, use the fdp engine:
// https://dreampuf.github.io/GraphvizOnline/#digraph%20G%20%7B%0A%20%20%20%20rq%20-%3E%20ch%2C%20sj%0Anf%20-%3E%20sm%2C%20rg%0Apc%20-%3E%20rz%2C%20zp%0Axt%20-%3E%20bc%0Ant%20-%3E%20kq%2C%20sj%0Ahc%20-%3E%20kb%2C%20zp%0Ard%20-%3E%20lk%0Aml%20-%3E%20pp%2C%20xt%0Asq%20-%3E%20kl%2C%20sj%0Ajg%20-%3E%20fl%2C%20rg%0Axl%20-%3E%20df%0Akl%20-%3E%20mb%2C%20sj%0And%20-%3E%20rg%2C%20jg%0Arg%20-%3E%20cs%2C%20zb%2C%20cp%2C%20vz%2C%20gp%0Amf%20-%3E%20zp%0Arz%20-%3E%20zp%2C%20fr%0Akk%20-%3E%20rg%2C%20bj%0Anb%20-%3E%20qj%0Apr%20-%3E%20pp%0Azp%20-%3E%20vl%2C%20lk%2C%20rd%2C%20kb%2C%20xl%0Afl%20-%3E%20nf%2C%20rg%0Atb%20-%3E%20pk%2C%20pp%0Abh%20-%3E%20pp%2C%20pr%0Anh%20-%3E%20sj%2C%20rq%0Alk%20-%3E%20hc%0Acp%20-%3E%20kk%0Aln%20-%3E%20df%0Axp%20-%3E%20df%0Abc%20-%3E%20nb%2C%20pp%0Alj%20-%3E%20rg%0Avz%20-%3E%20nd%0Avl%20-%3E%20lv%2C%20zp%0Agp%20-%3E%20df%0Ahd%20-%3E%20pp%2C%20bq%0Afq%20-%3E%20pp%2C%20bh%0Apk%20-%3E%20fq%2C%20pp%0Acs%20-%3E%20zb%2C%20rg%0Asn%20-%3E%20fd%0Akq%20-%3E%20sj%2C%20qq%0Azb%20-%3E%20vz%0Alv%20-%3E%20zp%2C%20rd%0Aqj%20-%3E%20pp%2C%20hd%0Afd%20-%3E%20nt%0Adf%20-%3E%20rx%0Abroadcaster%20-%3E%20vl%2C%20cs%2C%20cn%2C%20ml%0Abq%20-%3E%20tb%0Akb%20-%3E%20pc%0Acn%20-%3E%20sn%2C%20sj%0Aqq%20-%3E%20sq%0Amb%20-%3E%20sj%2C%20nh%0Ajd%20-%3E%20zp%2C%20mf%0Asj%20-%3E%20xp%2C%20qq%2C%20cn%2C%20fd%2C%20sn%0App%20-%3E%20ln%2C%20ml%2C%20xt%2C%20bq%2C%20nb%0Asm%20-%3E%20rg%2C%20cp%0Ach%20-%3E%20sj%0Abj%20-%3E%20lj%2C%20rg%0Afr%20-%3E%20zp%2C%20mr%0Amr%20-%3E%20zp%2C%20jd%0A%7D
// We now see that 'rx' is only connected to the conjunction 'df', which itself has 4 inputs: [xl, ln, xp, gp]
// So we need to find the cycles of these four outputting a high signal. Only when all these four output a high signal
// all at once will 'df' output a low signal, so we need to calculate the LCM of the four periods.
fn part2(input: &str) -> usize {
    let a = find_period("xl", input);  // xl at 4051
    let b = find_period("ln", input);  // ln at 4021
    let c = find_period("xp", input);  // xp at 4057
    let d = find_period("gp", input);  // gp at 3833

    let total_period = lcm(a, lcm(b, lcm(c, d)));
    println!("Part 2: requires {total_period} to send low signal to 'rx'. Individual periods: xl={a}, ln={b}, xp={c}, gp={d}");
    total_period as usize
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Module {
    label: String,
    module_type: ModuleType,
    state: bool,
    memory: HashMap<String, bool>,
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

    module_map.insert("button".to_string(), Module::new("button".to_string(), Button));
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

fn send_signal_bfs(connections: &HashMap<String, Vec<String>>, modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let (mut low, mut high) = (0, 0);

    let button = &"button".to_string();
    let broadcast = &"broadcaster".to_string();
    let mut signals_to_send = VecDeque::from([(button, broadcast, false)]);

    while let Some((from_name, current_name, high_signal)) = signals_to_send.pop_front() {
        if high_signal { high += 1 } else { low += 1 }
        // let string = if high_signal { "high" } else {"low"};
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
                    signal_to_send = !current.memory.values().all(|&b| b); // if all high send low, else send high
                }
                Output => {
                    // println!("Received {high_signal} signal from {from_name}");
                    continue;
                }
            }
            for next_node in &connections[current_name] {
                signals_to_send.push_back((current_name, next_node, signal_to_send));
                // println!("    - Next turn sending from {current_name} signal {signal_to_send} to {next_node}");
            }
        }
    }
    return (low, high);
}

fn find_period(looking_for: &str, input: &str) -> i64 {
    let (connections, mut modules) = parse(input);
    let mut button_clicks = 0;
    loop {
        let outputs_high_signal = send_signal_bfs_pt2(&connections, &mut modules, &looking_for.to_string());
        button_clicks += 1;

        if outputs_high_signal { return button_clicks; }
    }
}

fn send_signal_bfs_pt2(connections: &HashMap<String, Vec<String>>, modules: &mut HashMap<String, Module>, looking_for: &String) -> bool {
    let (button, broadcast) = (&"button".to_string(), &"broadcaster".to_string());
    let mut signals_to_send = VecDeque::from([(button, broadcast, false)]);

    while let Some((from_name, current_name, high_signal)) = signals_to_send.pop_front() {
        if current_name == &"df".to_string() && high_signal && from_name == looking_for {
            return true;
        }

        let signal_to_send;
        if let Some(current) = modules.get_mut(current_name) {
            match current.module_type {
                Button => unreachable!(),
                Broadcaster => signal_to_send = high_signal,
                FlipFlop => if !high_signal {
                    current.state = !current.state;
                    signal_to_send = current.state;
                } else {
                    continue;
                },
                Conjunction => {
                    let memory_entry = current.memory.entry(from_name.clone()).or_insert(high_signal);
                    *memory_entry = high_signal;
                    // if all high send low, else send high
                    signal_to_send = !current.memory.values().all(|&b| b);
                }
                Output => continue,
            }
            for next_node in &connections[current_name] {
                signals_to_send.push_back((current_name, next_node, signal_to_send));
            }
        }
    }
    return false;
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day20::{part1, part2};

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

    #[test]
    fn part_2_input() {
        let input = &fs::read_to_string("./inputs/day20/input.txt").unwrap();
        assert_eq!(part2(input), 253302889093151)
    }
}