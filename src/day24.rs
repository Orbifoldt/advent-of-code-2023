use std::fs;

use itertools::Itertools;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day24/input.txt").unwrap();
    part1(input, 200000000000000f64, 400000000000000f64);
    // part2(input);
}

fn part1(input: &str, min: f64, max: f64) -> usize {
    let hailstones = parse(input);
    let intersecting_combinations = hailstones.iter()
        .combinations(2)  // Gives all unique, unordered combinations
        .map(|pair| ((pair[0], pair[1]), intersection_2d(pair[0], pair[1])))
        .filter(|(_, ((x,y), (ta, tb)))|
            ta >= &0f64 && tb >= &0f64
                && &min <= x && x <= &max
                && &min <= y && y <= &max
        )
        // .inspect(|((a,b), ((x,y), (ta, tb)))| println!("Hailstone A {:?} and B {:?} instersect at ({x}, {y})", a, b))
        .count();
    println!("Part 1: {intersecting_combinations} hailstones intersect within the bounds {min} and {max}");
    intersecting_combinations
}

fn part2(input: &str) -> usize {
    todo!()
}


fn parse(input: &str) -> Vec<HailStone> {
    input.lines().map(|line| {
        let (position, velocity) = line.split(" @ ").tuple_windows().next().unwrap();
        let (x, y, z) = position.split(", ").map(|string| string.parse::<i64>().unwrap()).tuple_windows().next().unwrap();
        let (vx, vy, vz) = velocity.trim().split(", ")
            .map(|string| string.trim().parse::<i64>().unwrap())
            .tuple_windows().next().unwrap();
        HailStone { p: Vec3D { x, y, z}, v: Vec3D { x: vx, y: vy, z: vz } }
    }).collect_vec()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Vec3D {
    x: i64,
    y: i64,
    z: i64,
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
// (v_A | -v_B) * (t s)^T = p_B - p_A   (where (v_A | -v_B) is a 2x2 matrix with columns v_A and -v_B)
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

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::day24::{HailStone, intersection_2d, part1, Vec3D};

    #[test]
    fn part_1_intersection_function_should_return_intersection_point() {
        let ((x, y), _) = intersection_2d(
            &HailStone { p: Vec3D { x: 19, y: 13, z: 0}, v: Vec3D { x: -2, y: 1, z: 0}},
            &HailStone { p: Vec3D { x: 18, y: 19, z: 0}, v: Vec3D { x: -1, y: -1, z: 0}}
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
        assert_eq!(part1(input,7f64, 27f64), 2)
    }
}