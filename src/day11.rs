use std::convert::identity;
use std::fs;

pub fn main() {
    let input = &fs::read_to_string("./inputs/day11/input.txt").unwrap();
    part1(input);
    // part2(input);
}

fn part1(input: &str) -> i64 {
    let image = parse(input);
    let expanded =  expand(image);
    let galaxy_coordinates = &expanded.iter().enumerate().flat_map(|(y, row)|
        row.iter().enumerate().filter_map(move |(x, b)| b.then_some((x, y)))
    ).collect::<Vec<_>>();

    let mut sum = 0;
    for (ax, ay) in galaxy_coordinates {
        for (bx, by) in galaxy_coordinates {
            sum += ax.abs_diff(*bx) + ay.abs_diff(*by);
        }
    }

    println!("Part 1: sum of distances of pairs of galaxies is {}", sum / 2);
    sum as i64 / 2
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line| line.chars().map(|c| c == '#').collect()).collect()
}

fn expand(image: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let empty_rows = image.iter().enumerate()
        .filter_map(|(y, row)| (!row.iter().cloned().any(identity)).then_some(y))
        .collect::<Vec<_>>();
    let empty_columns = image[0].iter()
        .enumerate().filter_map(|(x, _)|
        (!image.iter().map(|row| row[x]).any(identity)).then_some(x)
    ).collect::<Vec<_>>();

    // image.iter().for_each(|row| println!("{:?}", row));
    // empty_rows.iter().for_each(|y| println!("Row {y} is empty"));
    // empty_columns.iter().for_each(|x| println!("column {x} is empty"));

    let expanded_map = image.iter().enumerate().rev()
        .flat_map(|(y, row)| {
            let new_row = row.iter().enumerate().rev()
                .flat_map(|(x, &b)| if empty_columns.contains(&x) { vec![b, b] } else { vec![b] })
                .rev()
                .collect::<Vec<bool>>();

            if empty_rows.contains(&y) {
                let cloned_row = new_row.clone();
                vec![new_row, cloned_row]
            } else {
                vec![new_row]
            }
        })
        .rev()
        .collect::<Vec<Vec<bool>>>();
    // expanded_map.iter().for_each(|row| println!("{:?}", row));
    expanded_map
}


#[cfg(test)]
mod tests {
    use crate::day11::part1;

    #[test]
    fn test() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part1(input), 374)
    }
}