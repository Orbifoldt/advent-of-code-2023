use std::cmp::{max, Ordering};
use std::collections::{HashMap, HashSet};
use std::convert::identity;
use std::fs;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day23/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> usize {
    let board = parse(input);
    let (width, height) = (board[0].len(), board.len());
    let start = (1, 0);
    let end = (width - 2, height - 1);

    let mut stack = vec![(HashSet::from([start]), start)];
    let mut max_dist = 0;

    while let Some((path, current)) = stack.pop() {
        for next in next_tiles(&board, current) {
            if next == end {
                max_dist = max(max_dist, path.len())
            } else if !path.contains(&next) {
                let mut new_path = path.clone();
                new_path.insert(next);
                stack.push((new_path, next));
            }
        }
    }

    println!("Part 1: longest walk you can take from {:?} to {:?} has length {max_dist}", start, end);
    max_dist
}


fn part2(input: &str) -> usize {
    let (graph, start, end) = parse_pt2(input);

    // DFS
    let mut max_dist = 0;
    let mut total_paths_count = 0;
    let mut stack = vec![(HashSet::from([start]), 0usize, start)];
    while let Some((visited_nodes, path_length, current)) = stack.pop() {
        for (next, distance) in graph.adjacency.get(&current).unwrap() {
            let new_path_length = path_length + (*distance as usize);
            if next == &end {
                max_dist = max(max_dist, new_path_length);
                total_paths_count += 1;
                if total_paths_count % 100_000 == 0 { println!("Reached end {total_paths_count} times, max so far is {max_dist}") }
            } else if !visited_nodes.contains(&next){
                let mut new_path = visited_nodes.clone();
                new_path.insert(*next);
                stack.push((new_path, new_path_length, *next));
            }
        }
    }

    println!("Part 2: longest walk you can take from {:?} to {:?} has length {max_dist}", start, end);
    max_dist
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect_vec()).collect_vec()
}

fn parse_pt2(input: &str) -> (Graph, Node, Node) {
    let field = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let (width, height) = (field[0].len() as u8, field.len() as u8);
    let start = Node { x: 1, y: 0 };
    let end = Node { x: width - 2, y: height - 1 };

    let mut nodes = HashSet::from([start]);
    let mut edges = HashSet::new();

    let mut node_stack = vec![start];
    while let Some(current_node) = node_stack.pop() {
        let start = (current_node.x, current_node.y);
        for start_neighbor in next_tiles_pt2(&field, start, (width, height)) {
            let mut current = start_neighbor;
            let mut previous = start.clone();
            let mut distance = 0;

            loop {
                distance += 1;
                let neighbors = next_tiles_pt2(&field, current, (width, height)).into_iter()
                    .filter(|other| other != &current && other != &previous)
                    .collect_vec();

                if neighbors.len() == 1 {
                    previous = current;
                    current = neighbors[0];
                } else {
                    let found_node = Node { x: current.0, y: current.1 };
                    let is_new_node = nodes.insert(found_node);
                    edges.insert(Edge::new(current_node.clone(), found_node.clone(), distance));

                    if is_new_node {  // When new node is discovered we push it to also explore it's edges
                        node_stack.push(found_node)
                    }
                    break;
                }
            }
        }
    }

    (Graph::new(nodes, edges), start, end)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Node {
    x: u8,
    y: u8,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            ord => ord,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Undirected, weighted edge in a graph
#[derive(Copy, Clone, Debug)]
struct Edge {
    a: Node,
    b: Node,
    weight: u32,
}

impl Edge {
    pub fn new(node_1: Node, node_2: Node, weight: u32) -> Self {
        if node_2 < node_1 {
            Self { a: node_2, b: node_1, weight }
        } else {
            Self { a: node_1, b: node_2, weight }
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
            && ((self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a) )
    }
}
impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.a.hash(hasher);
        self.b.hash(hasher);
        self.weight.hash(hasher);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Graph {
    nodes: HashSet<Node>,
    edges: HashSet<Edge>,
    adjacency: HashMap<Node, HashSet<(Node, u32)>>,
}

impl Graph {
    pub fn new(nodes: HashSet<Node>, edges: HashSet<Edge>) -> Self {
        let adjacency = nodes.iter().map(|node| {
            let neighbors = edges.iter().filter_map(|edge|
                if &edge.a == node {
                    Some((edge.b.clone(), edge.weight))
                } else if &edge.b == node {
                    Some((edge.a.clone(), edge.weight))
                } else {
                    None
                }
            ).collect::<HashSet<(Node, u32)>>();
            (*node, neighbors)
        }).collect::<HashMap<Node, HashSet<(Node, u32)>>>();
        Self { nodes, edges, adjacency }
    }
}


// Returns valid tiles to step onto from the current position
fn next_tiles(field: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    if y == 0 {
        return [(x + 1, y), (x - 1, y), (x, y + 1)].into_iter()
            .filter(|(x, y)| field[*y][*x] != '#')
            .collect_vec();
    }
    // We assert here that the input is "nice", e.g. no slopes pointing into walls, and all neighbors within bounds
    let cur = field[y][x];
    match cur {
        '>' => vec![(x + 1, y)],
        'v' => vec![(x, y + 1)],
        '<' => vec![(x - 1, y)],
        '^' => vec![(x, y - 1)],
        _ => [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
            .filter(|(x, y)| field[*y][*x] != '#')
            .collect_vec()
    }
}

// Returns valid tiles to step onto from the current position
fn next_tiles_pt2(field: &Vec<Vec<char>>, (x, y): (u8, u8), (width, height): (u8, u8)) -> Vec<(u8, u8)> {
    [
        if x < width - 1 { Some((x + 1, y)) } else { None },
        if x > 0 { Some((x - 1, y)) } else { None },
        if y < height - 1 { Some((x, y + 1)) } else { None },
        if y > 0 { Some((x, y - 1)) } else { None }
    ]
        .into_iter()
        .filter_map(identity)
        .filter(|(x, y)| field[*y as usize][*x as usize] != '#')
        .collect_vec()
}


#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashSet;
    use std::fs;
    use std::hash::{Hash, Hasher};
    use crate::day23::{Edge, Graph, Node, parse_pt2, part1, part2};

    #[test]
    fn part_1_simple_example() {
        let input = r"#.#####################
#.......###############
#######.###############
#######...............#
#####################.#";
        assert_eq!(part1(input), 24)
    }

    #[test]
    fn part_1_example_1() {
        let input = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(part1(input), 94)
    }

    #[test]
    fn part_1_input() {
        let input = &fs::read_to_string("./inputs/day23/input.txt").unwrap();
        assert_eq!(part1(input), 2306)
    }

    #[test]
    fn part_2_edges_should_be_unordered_pairs() {
        let a = Node { x: 1, y: 0 };
        let b = Node { x: 8, y: 4 };
        let edge1 = Edge::new(a, b, 5);
        let edge2 = Edge::new(b, a, 5);
        assert!(edge1.eq(&edge2));
        assert!(edge2.eq(&edge1));

        fn hash(edge1: Edge) -> u64 {
            let mut hasher = DefaultHasher::new();
            edge1.hash(&mut hasher);
            hasher.finish()
        }
        let hash1 = hash(edge1);
        let hash2 = hash(edge2);
        assert_eq!(hash1, hash2);

        let set = HashSet::from([edge1, edge2]);
        assert_eq!(set.iter().count(), 1);
    }


    #[test]
    fn part_2_parse_to_graph() {
        let input = r"#.########
#......###
###.##.###
###......#
########.#";
        let (graph, start, end) = parse_pt2(input);
        let expected_start = Node { x: 1, y: 0 };
        let expected_end = Node { x: 8, y: 4 };
        let node1 = Node { x: 3, y: 1 };
        let node2 = Node { x: 6, y: 3 };
        assert_eq!(start, expected_start);
        assert_eq!(end, expected_end);
        assert_eq!(graph.nodes, HashSet::from([start, end, node1, node2]));
        assert!(graph.edges.contains(&Edge::new(start, node1, 3)));
        assert!(graph.edges.contains(&Edge::new(node1, node2, 5)));
        assert!(graph.edges.contains(&Edge::new(node2, end, 3)));
        assert_eq!(graph.adjacency[&start], HashSet::from([(node1, 3)]));
        assert_eq!(graph.adjacency[&node1], HashSet::from([(start, 3), (node2,5)]));
        assert_eq!(graph.adjacency[&node2], HashSet::from([(node1, 5), (end, 3)]));
        assert_eq!(graph.adjacency[&end], HashSet::from([(node2, 3)]));
    }

    #[test]
    fn part_2_example_1() {
        let input = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(part2(input), 154)
    }

    #[test]
    fn part_2_input() {
        let input = &fs::read_to_string("./inputs/day23/input.txt").unwrap();
        assert_eq!(part2(input), 6718)  // runs kinda slow
    }
}