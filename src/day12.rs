use std::collections::HashMap;
use std::fs;
use std::iter::repeat;
use itertools::Itertools;
use crate::common::{get_numbers, split_first};

pub fn main() {
    let input = &fs::read_to_string("./inputs/day12/input.txt").unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> usize {
    let sum_configs = input.lines().map(|line|
        num_valid_configs(line)
    ).sum();
    println!("Part 1: sum of all possible configurations is {sum_configs}");
    sum_configs
}

fn part2(input: &str) -> usize {
    let sum_configs = input.lines().map(|line|
        num_valid_configs(unfold(line).as_str())
    ).sum();
    println!("Part 2: sum of all possible configurations is {sum_configs}");
    sum_configs
}

fn unfold(folded_line: &str) -> String {
    let (config_str, groups_str) = split_first(folded_line, ' ').unwrap();
    let unfolded_config = repeat(config_str).take(5).intersperse("?").collect::<String>();
    let unfolded_groups = repeat(groups_str).take(5).intersperse(",").collect::<String>();
    unfolded_config + " " + &*unfolded_groups
}

fn num_valid_configs(springs_line: &str) -> usize {
    let (config_str, groups_str) = split_first(springs_line, ' ').unwrap();
    let groups = get_numbers::<usize>(groups_str.replace(",", " ").as_str());
    valid_configs(config_str, &groups)
}

fn valid_configs(spring_config: &str, groups: &Vec<usize>) -> usize {
    valid_configs_cached(spring_config, groups, 0, 0, 0, &mut HashMap::new())
}

fn valid_configs_cached(spring_config: &str, groups: &Vec<usize>, cur_idx: usize, cur_group_idx: usize, cur_group_size: usize, mut cache: &mut HashMap<(usize, usize, usize), usize>) -> usize {
    if let Some(cached_value) = cache.get(&(cur_idx, cur_group_idx, cur_group_size)) {
        return *cached_value;
    }

    if cur_idx == spring_config.len() { // reached the end
        return if cur_group_idx == groups.len() && cur_group_size == 0 { // outside a group
            1
        } else if cur_group_idx == groups.len() - 1 && cur_group_size == groups[cur_group_idx] { // inside last group that exactly matches desired group
            1
        } else { // ended up in invalid state: too many/little groups, or last group incorrect size
            0
        };
    }

    let cur_char = spring_config.as_bytes()[cur_idx] as char;  // our input is ascii, so this works
    let bools_to_check = if cur_char == '?' { vec![true, false] } else { vec![cur_char == '#'] };
    let mut local_count = 0;
    for b in bools_to_check {
        if b { // inside or entering a group
            local_count += valid_configs_cached(spring_config, groups, cur_idx + 1, cur_group_idx, cur_group_size + 1, cache);
        } else if !b && cur_group_size == 0 { // outside a group
            local_count += valid_configs_cached(spring_config, groups, cur_idx + 1, cur_group_idx, 0, cache);
        } else if !b && cur_group_size > 0 { // at the end of a group
            if cur_group_idx < groups.len() && groups[cur_group_idx] == cur_group_size {
                local_count += valid_configs_cached(spring_config, groups, cur_idx + 1, cur_group_idx + 1, 0, cache);
            }
        } else {
            // no valid possibilities, continue... This includes groups running too long
        }
    }
    cache.insert((cur_idx, cur_group_idx, cur_group_size), local_count);
    local_count
}


#[cfg(test)]
mod tests {
    use crate::day12::{num_valid_configs, part1, part2, unfold};

    #[test]
    fn correctly_determine_number_of_configurations() {
        assert_eq!(num_valid_configs("???.### 1,1,3"), 1);
        assert_eq!(num_valid_configs(".??..??...?##. 1,1,3"), 4);
        assert_eq!(num_valid_configs("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(num_valid_configs("????.#...#... 4,1,1"), 1);
        assert_eq!(num_valid_configs("????.######..#####. 1,6,5"), 4);
        assert_eq!(num_valid_configs("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn example_part1() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part1(input), 21)
    }

    #[test]
    fn should_correctly_unfold_a_line() {
        assert_eq!(unfold(".# 1"), ".#?.#?.#?.#?.# 1,1,1,1,1");
        assert_eq!(
            unfold("???.### 1,1,3"),
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3",
        );
    }

    #[test]
    fn example_part2() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part2(input), 525152)
    }

    #[test]
    fn example_part2_line1() {
        let input = r"???.### 1,1,3";
        assert_eq!(part2("???.### 1,1,3"), 1)
    }

    #[test]
    fn example_part2_line2() {
        assert_eq!(part2(".??..??...?##. 1,1,3"), 16384)
    }
}