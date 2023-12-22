use std::cmp::min;
use std::collections::HashSet;
use std::convert::identity;
use std::fmt::{Display, Formatter, Write};
use std::fs;

use itertools::Itertools;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day22/input.txt").unwrap();
    part_1_and_2(input);
}

fn part_1_and_2(input: &str) -> (usize, usize) {
    let (stable_count, chain_reaction_count) = let_them_fall(input);
    println!("Part 1: {stable_count} bricks can be disintegrated without others falling down");
    println!("Part 2: Sum of chain reaction count is {chain_reaction_count}");
    (stable_count, chain_reaction_count)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Cube { x: u16, y: u16, z: u16 }

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Brick { a: Cube, b: Cube }

fn let_them_fall(input: &str) -> (usize, usize) {
    let bricks_input = parse(input); // already sorted
    let (cubes, bricks, _) = settle_bricks(bricks_input);

    let mut num_bricks_that_can_safely_be_removed = 0;
    let mut chain_reaction_counter = 0;
    for brick in &bricks {
        // println!("\nSeeing what happens when removing {brick}");
        let brick_cubes = brick.cubes_vec();
        let cubes_without_current_brick = cubes.clone().into_iter()
            .filter(|cube| !brick_cubes.contains(cube)).collect();

        let bricks_that_fall = bricks.iter()
            .filter(|&other_brick| other_brick != brick)
            .filter(|&other_brick| { other_brick.can_fall_down(&cubes_without_current_brick) })
            .collect_vec();

        if bricks_that_fall.is_empty() {
            num_bricks_that_can_safely_be_removed += 1;
        } else {
            let bricks_without_current_brick = bricks.clone().into_iter()
                .filter(|b| b != brick)
                .sorted_by_key(|brick| min(brick.a.z, brick.b.z))
                .collect_vec();
            let (_, _, count) = settle_bricks(bricks_without_current_brick);
            chain_reaction_counter += count;
        }
    }
    (num_bricks_that_can_safely_be_removed, chain_reaction_counter)
}

fn settle_bricks(bricks: Vec<Brick>) -> (HashSet<Cube>, HashSet<Brick>, usize) {
    let mut settled_cubes: HashSet<Cube> = HashSet::new();
    let mut settles_bricks: HashSet<Brick> = HashSet::new();
    let mut number_bricks_that_moved = 0;

    for mut brick in bricks {
        let mut moved = false;
        loop {
            if brick.can_fall_down(&settled_cubes) {
                brick.drop_down();
                moved = true;
            } else {
                brick.cubes_vec().into_iter().for_each(|cube| { settled_cubes.insert(cube); });
                settles_bricks.insert(brick);
                break;
            }
        }
        if moved { number_bricks_that_moved += 1 }
    }
    (settled_cubes, settles_bricks, number_bricks_that_moved)
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("({}, {}, {})", self.x, self.y, self.z))
    }
}

impl Cube {
    fn from_str(string: &str) -> Cube {
        let [x, y, z] = string.split(",").map(|str| str.parse::<u16>().unwrap()).collect_vec()[..] else { unreachable!() };
        Cube { x, y, z }
    }

    fn as_one_down(&self) -> Cube { Cube { x: self.x, y: self.y, z: self.z - 1 } }
}

impl Display for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("Brick[{}, {}]", self.a, self.b))
    }
}

impl Brick {
    fn cubes_vec(&self) -> Vec<Cube> {
        (self.a.x..=self.b.x).flat_map(|x|
            (self.a.y..=self.b.y).flat_map(|y|
                (self.a.z..=self.b.z).map(|z|
                    Cube { x, y, z }
                ).collect_vec()
            ).collect_vec()
        ).collect_vec()
    }

    fn can_fall_down(&self, fallen_cubes: &HashSet<Cube>) -> bool {
        // for each cube in the brick check if it can fall down any further
        self.cubes_vec().into_iter().map(|cube| {
            if cube.z > 0 {
                let one_down = cube.as_one_down();
                if self.cubes_vec().contains(&one_down) // for a vertical brick, need to exclude case where cubes collide with other cubes in this brick
                    || !fallen_cubes.contains(&one_down)
                {
                    return true;
                }
            }
            false
        }).all(identity)
    }

    fn drop_down(&mut self) {
        self.a.z -= 1;
        self.b.z -= 1;
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input.lines().map(|line| {
        let comps = line.split("~").collect::<Vec<_>>();
        Brick { a: Cube::from_str(comps[0]), b: Cube::from_str(comps[1]) }
    })
        .sorted_by_key(|brick| min(brick.a.z, brick.b.z))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::day22::part_1_and_2;

    #[test]
    fn part_1_simple_example_1() {
        let input = r"0,0,3~1,0,3
0,0,5~0,1,5";
        assert_eq!(part_1_and_2(input).0, 1)
    }

    #[test]
    fn part_1_simple_example_2() {
        let input = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3"; // just A, B and C
        assert_eq!(part_1_and_2(input).0, 2)
    }

    #[test]
    fn part_1_simple_example_3() {
        let input = r"0,1,6~2,1,6
1,1,8~1,1,9"; // just F & G
        assert_eq!(part_1_and_2(input).0, 1)
    }

    #[test]
    fn part_1_example() {
        let input = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(part_1_and_2(input).0, 5)
    }

    #[test]
    fn part_2_example() {
        let input = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(part_1_and_2(input).1, 7)
    }
}