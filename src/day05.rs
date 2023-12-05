pub fn main(){

}

pub fn part1(input: &str) -> i64 {
    35
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day05::part1;

    #[test]
    fn should_return_correct_number_of_cards_for_part2() {
        let lowest_location = part1(&fs::read_to_string("./day05/input_example.txt").unwrap());
        assert_eq!(lowest_location, 35)
    }
}