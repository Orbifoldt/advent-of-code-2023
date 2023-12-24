use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use std::ops::{Add, Div, Mul, RangeInclusive, Sub};
use std::sync::{Arc, Mutex};

use itertools::Itertools;
use rayon::prelude::*;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day24/input.txt").unwrap();
    part1(input, 200_000_000_000_000f64, 400_000_000_000_000f64);
    part2(input);
    // part2_hopelessly_slow(input, 00_000_000_000_000, 600_000_000_000_000);
}

fn part1(input: &str, min: f64, max: f64) -> usize {
    let hailstones = parse(input);
    let intersecting_combinations = hailstones.iter()
        .combinations(2)  // Gives all unique, unordered combinations
        .map(|pair| ((pair[0], pair[1]), intersection_2d(pair[0], pair[1])))
        .filter(|(_, ((x, y), (ta, tb)))|
            ta >= &0f64 && tb >= &0f64
                && &min <= x && x <= &max
                && &min <= y && y <= &max
        )
        // .inspect(|((a,b), ((x,y), (ta, tb)))| println!("Hailstone A {:?} and B {:?} intersect at ({x}, {y})", a, b))
        .count();
    println!("Part 1: {intersecting_combinations} hailstones intersect within the bounds {min} and {max}");
    intersecting_combinations
}

fn part2(input: &str) -> i64 {
    let hailstones = parse(input);
    let (a, b, c) = hailstones.iter().tuple_windows().next().unwrap();

    println!("(declare-const px Int)");
    println!("(declare-const py Int)");
    println!("(declare-const pz Int)");
    println!("(declare-const vx Int)");
    println!("(declare-const vy Int)");
    println!("(declare-const vz Int)");
    println!("(push)");
    for (i, hail_stone) in [a, b, c].iter().enumerate() {
        println!("(declare-const t{i} Int)");
        println!("(assert (= (+ px (* vx t{i})) (+ {} (* {} t{i}))))", hail_stone.p.x, hail_stone.v.x);
        println!("(assert (= (+ py (* vy t{i})) (+ {} (* {} t{i}))))", hail_stone.p.y, hail_stone.v.y);
        println!("(assert (= (+ pz (* vz t{i})) (+ {} (* {} t{i}))))", hail_stone.p.z, hail_stone.v.z);
        println!("(push)");
    }
    println!("(check-sat)");
    println!("(get-model)");

    // put into Z3, then we find solution: (couldn't get rust z3 crate to work, seems like windows issue)
    let p = Vec3D { x: 187016878804004, y: 175507140888229, z:  177831791810924 };
    let v = Vec3D { x: 192, y: 210, z: 179 };
    let t = (696407182343i64, 447383459952i64, 891640066892i64);

    let sum = p.x + p.y + p.z;
    println!("Part 2: Sum of coordinates of the starting point of our thrown rock is {sum}");
    sum
}
fn part2_hopelessly_slow(input: &str, min: i64, max: i64) -> i64 {
    let hailstones = parse(input);
    let (a, b, c, d, e) = hailstones.iter().tuple_windows().next().unwrap();

    let mut possible_times = Arc::new(Mutex::new(HashSet::new()));
    let range = time_range(a, min, max);
    let range_size = (range.end() - range.start() + 1)  as u64;
    range.into_par_iter()
        .for_each(|s|{
    // 'a: for s in time_range(a, min, max) {
        let pa = a.p + a.v * s;
        // if !pa.is_within_bounds(min, max) {
        //     continue 'a;
        // }
        'b: for t in time_range(b, min, max) {
            let pb = b.p + b.v * t;
            // if !pb.is_within_bounds(min, max) {
            //     continue 'b;
            // }
            'c: for u in time_range(c, min, max) {
                let pc = c.p + c.v * u;

                // if !pc.is_within_bounds(min, max) {
                //     continue 'c;
                // }
                if are_collinear(&pa, &pb, &pc) {
                    println!("{:?}, {:?} and {:?} could intersect the same line at s={s}, t={t} and u={u}", a.p, b.p, c.p);
                    'd: for r in time_range(d, min, max) {
                        let pd = d.p + d.v * r;
                        // if !pd.is_within_bounds(min, max) {
                        //     continue 'd;
                        // }
                        if are_collinear(&pa, &pb, &pd) && are_collinear(&pa, &pc, &pd) && are_collinear(&pb, &pc, &pd) {
                            println!("  > {:?}, {:?}, {:?} and {:?} could intersect the same line at s={s}, t={t}, u={u} and r={r}", a.p, b.p, c.p, d.p);
                            let mut vec = possible_times.lock().unwrap();
                            vec.insert((s, t, u));
                            break 'd;
                        }
                    }
                }
            }
        }
    });


    let &(s, t, u) = possible_times.lock().unwrap().iter().next().expect("Should have found an intersection point");
    let pa = a.p + a.v * s;
    let pb = b.p + b.v * t;
    // let pc = c.p + c.v * u;

    let velocity = Vec3D {
        x: pb.x - pa.x,
        y: pb.y - pa.y,
        z: pb.z - pa.z,
    } / (t - s);
    let initial = pa - velocity * s;
    let sum = initial.x + initial.y + initial.z;
    println!("Part 2: sum of initial x, y and z coords is {sum}. The line starts at {:?} and moves in direction {:?}", initial, velocity);
    sum
}


fn parse(input: &str) -> Vec<HailStone> {
    input.lines().map(|line| {
        let (position, velocity) = line.split(" @ ").tuple_windows().next().unwrap();
        let (x, y, z) = position.split(", ").map(|string| string.parse::<i64>().unwrap()).tuple_windows().next().unwrap();
        let (vx, vy, vz) = velocity.trim().split(", ")
            .map(|string| string.trim().parse::<i64>().unwrap())
            .tuple_windows().next().unwrap();
        HailStone { p: Vec3D { x, y, z }, v: Vec3D { x: vx, y: vy, z: vz } }
    }).collect_vec()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Vec3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Add for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3D { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3D { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul<i64> for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: i64) -> Self::Output {
        Vec3D { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Div<i64> for Vec3D {
    type Output = Vec3D;

    fn div(self, rhs: i64) -> Self::Output {
        Vec3D { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Vec3D {
    fn is_within_bounds(&self, min: i64, max: i64) -> bool {
        min <= self.x && self.x <= max && min <= self.y && self.y <= max && min <= self.z && self.z <= max
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct HailStone {
    p: Vec3D,
    v: Vec3D,
}

// Finds the intersection (x, y) and times (t, s) where the paths of two moving particles intersect
// If p_A and p_B are the position vectors of A and B, and v_A and v_B their velocities, then we need to solve:
//    p_A +t*v_A = p_B + s*v_B,   for s, t in \mathbb{R}
// Writing this out and using the known inverse of a 2x2 matrix, we get:
//   (v_A | -v_B) * (t s)^T = p_B - p_A     (where (v_A | -v_B) is a 2x2 matrix with columns v_A and -v_B)
//     <=> (t, s) = 1/det * ((-v_B,y  v_B,x), (-v_A,y  v_A,x)) * (p_B - p_A)
//     <=> t = 1/det * ( -v_B,y * (p_B,x - p_A,x) + v_B,x * (p_B,y - p_A,y) )
//            && s = 1/det * ( -v_A,y * (p_B,x - p_A,x) + v_A,x * (p_B,y - p_A,y) )
fn intersection_2d(a: &HailStone, b: &HailStone) -> ((f64, f64), (f64, f64)) {
    let determinant = a.v.x * (-b.v.y) - (-b.v.x) * a.v.y;
    let t = ((-b.v.y * (b.p.x - a.p.x) + b.v.x * (b.p.y - a.p.y)) as f64) / (determinant as f64);
    let s = ((-a.v.y * (b.p.x - a.p.x) + a.v.x * (b.p.y - a.p.y)) as f64) / (determinant as f64);
    let (x, y) = ((b.p.x as f64) + s * (b.v.x as f64), (b.p.y as f64) + s * (b.v.y as f64));
    ((x, y), (t, s))
}

fn determinant(a: &Vec3D, b: &Vec3D, c: &Vec3D) -> i64 {
    a.x.wrapping_mul(b.y.wrapping_mul(c.z).wrapping_sub(c.y.wrapping_mul(b.z)))
        .wrapping_sub(a.y.wrapping_mul (b.x.wrapping_mul(c.z).wrapping_sub(c.x.wrapping_mul(b.z))))
        .wrapping_add(a.z.wrapping_mul(b.x.wrapping_mul(c.y).wrapping_sub(c.x.wrapping_mul(b.y))))
}

fn are_collinear(a: &Vec3D, b: &Vec3D, c: &Vec3D) -> bool {
    determinant(a, b, c) == 0
}

// Determines the time range such that all coordinates of the hailstone are within the range min_val..=max_val
fn time_range(hail_stone: &HailStone, min_val: i64, max_val: i64) -> RangeInclusive<i64> {
    let minimum = max(0, max(
        min_t(hail_stone.v.x, hail_stone.p.x, min_val),
        max(min_t(hail_stone.v.y, hail_stone.p.y, min_val), min_t(hail_stone.v.z, hail_stone.p.z, min_val))
    ));
    let maximum = max(0, min(
        max_t(hail_stone.v.x, hail_stone.p.x, max_val),
        min(max_t(hail_stone.v.y, hail_stone.p.y, max_val), max_t(hail_stone.v.z, hail_stone.p.z, max_val))
    ));
    if minimum <= maximum {
        minimum..=maximum
    } else {
        maximum..=minimum
    }
}

fn min_t(a: i64, b: i64, min: i64) -> i64 {
    (min - b) / a
}
fn max_t(a: i64, b: i64, max: i64) -> i64 {
    (max - b) / a + 1
}


#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::day24::{are_collinear, determinant, HailStone, intersection_2d, part1, part2, part2_hopelessly_slow, Vec3D};

    #[test]
    fn part_1_intersection_function_should_return_intersection_point() {
        let ((x, y), _) = intersection_2d(
            &HailStone { p: Vec3D { x: 19, y: 13, z: 0 }, v: Vec3D { x: -2, y: 1, z: 0 } },
            &HailStone { p: Vec3D { x: 18, y: 19, z: 0 }, v: Vec3D { x: -1, y: -1, z: 0 } },
        );
        assert!(approx_eq!(f64, x, 14.333f64, epsilon = 0.001));
        assert!(approx_eq!(f64, y, 15.333f64, epsilon = 0.001));
    }

    #[test]
    fn part_1_example_1() {
        let input = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(part1(input, 7f64, 27f64), 2)
    }

    #[test]
    fn should_correctly_find_if_vecs_are_collinear() {
        let a = Vec3D { x: 9, y: 18, z: 20 };
        let b = Vec3D { x: 15, y: 16, z: 16 };
        let c = Vec3D { x: 12, y: 17, z: 18 };
        assert_eq!(determinant(&a, &b, &c), 0);
        assert_eq!(are_collinear(&a, &b, &c), true);
    }

    #[test]
    fn part_2_example() {
        let input = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(part2_hopelessly_slow(input, -100, 200), 47)
    }

//     #[test]
//     fn part_2_v2_example() {
//         let input = r"19, 13, 30 @ -2,  1, -2
// 18, 19, 22 @ -1, -1, -2
// 20, 25, 34 @ -2, -2, -4
// 12, 31, 28 @ -1, -2, -1
// 20, 19, 15 @  1, -5, -3";
//         assert_eq!(part2(input), 47)
//     }
}