use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

pub fn main() {
    let content = fs::read_to_string("./inputs/day02/input.txt")
        .expect("Should be able to read the file");
    let games: Vec<Game> = content.lines().map(|line| read(line)).collect();

    let max = Subset { red: 12, green: 13, blue: 14, };
    part1(&games, &max);
    part2(&games);
}

fn part1(games: &Vec<Game>, max: &Subset) {
    let sum: i32 = games.iter().filter_map(|game| {
        if is_valid(game, &max) {
            Some(game.id)
        } else {
            //println!("Game {} is invalid: {:?}", game.id, game);
            None
        }
    }).sum();
    println!("Sum of invalid is {sum}")
}

fn part2(games: &Vec<Game>) {
    let sum: i32 = games.iter().map(|game| power(&fewest_necessary(game))).sum();
    println!("Sum of power of the minimum required sets is {sum}")
}

#[derive(Eq, PartialEq, Debug)]
struct Game {
    id: i32,
    subsets: Vec<Subset>,
}

#[derive(Eq, PartialEq, Debug)]
struct Subset {
    red: i32,
    blue: i32,
    green: i32,
}

impl Game {
    pub fn new(id: i32) -> Self {
        Self {
            id: id,
            subsets: Vec::new(),
        }
    }
}

lazy_static! {
    static ref GAME_ID_REGEX: Regex = Regex::new(r"Game (\d+):.*").unwrap();
    static ref BLUE_REGEX: Regex =  Regex::new(r"\s(\d+) blue").unwrap();
    static ref RED_REGEX: Regex =   Regex::new(r"\s(\d+) red").unwrap();
    static ref GREEN_REGEX: Regex = Regex::new(r"\s(\d+) green").unwrap();
}

pub fn read(game_string: &str) -> Game {
    let extracted: i32 = GAME_ID_REGEX.captures(game_string).unwrap()[1]
        .parse().unwrap();
    let mut game = Game::new(extracted);
    game_string.split(";").fold(&mut game, |game: &mut Game, subset_string| {
        let r: i32 = RED_REGEX.captures(subset_string).map_or(0, |x| x[1].parse().unwrap());
        let g: i32 = GREEN_REGEX.captures(subset_string).map_or(0, |x| x[1].parse().unwrap());
        let b: i32 = BLUE_REGEX.captures(subset_string).map_or(0, |x| x[1].parse().unwrap());
        game.subsets.push(Subset { red: r, blue: b, green: g });
        game
    });
    game
}

pub fn is_valid(game: &Game, max: &Subset) -> bool {
    game.subsets.iter().all(|subset|
        subset.red <= max.red
            && subset.green <= max.green
            && subset.blue <= max.blue
    )
}

pub fn fewest_necessary(game: &Game) -> Subset {
    Subset {
        red: game.subsets.iter().max_by_key(|subset| subset.red).unwrap().red,
        green: game.subsets.iter().max_by_key(|subset| subset.green).unwrap().green,
        blue: game.subsets.iter().max_by_key(|subset| subset.blue).unwrap().blue, }
}

pub fn power(subset: &Subset) -> i32 {
    subset.red * subset.green * subset.blue
}

#[cfg(test)]
mod tests {
    use crate::day02;
    use crate::day02::{fewest_necessary, Game, is_valid, power, Subset};

    const SINGLE_ROUND_GAME_STR: &str = "Game 32: 3 blue, 4 red, 27 green";
    const SAMPLE_GAME_STR: &str = "Game 7: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    fn sample_game() -> Game {
        Game {
            id: 7,
            subsets: vec![
                Subset { red: 4, green: 0, blue: 3 },
                Subset { red: 1, green: 2, blue: 6 },
                Subset { red: 0, green: 2, blue: 0 },
            ],
        }
    }

    #[test]
    fn should_correctly_parse_a_the_game_id() {
        let game = day02::read(SINGLE_ROUND_GAME_STR);
        assert_eq!(game.id, 32);
    }

    #[test]
    fn should_correctly_extract_red_from_a_single_round_game() {
        let game = day02::read(SINGLE_ROUND_GAME_STR);
        assert_eq!(game.subsets[0].red, 4);
    }

    #[test]
    fn subset_should_have_count_0_for_red_if_it_is_not_mentioned_in_the_string() {
        let game = day02::read("Game 32: 3 blue, 27 green");
        assert_eq!(game.subsets[0].red, 0);
    }

    #[test]
    fn should_extract_a_subset_from_a_single_round_game() {
        let game = day02::read(SINGLE_ROUND_GAME_STR);
        assert_eq!(game.subsets[0], Subset { red: 4, green: 27, blue: 3 });
    }

    #[test]
    fn should_extract_correct_number_of_subsets_for_multi_round_game() {
        let game = day02::read(SAMPLE_GAME_STR);
        assert_eq!(game.subsets.len(), 3);
    }

    #[test]
    fn should_extract_all_subsets_of_a_multi_round_game() {
        let game = day02::read(SAMPLE_GAME_STR);
        assert_eq!(game, sample_game());
    }

    #[test]
    fn when_all_below_maximum_game_should_be_valid() {
        let valid = is_valid(&sample_game(), &Subset { red: 4, green: 2, blue: 6,  });
        assert!(valid, "Game wasn't valid!")
    }

    #[test]
    fn when_red_is_too_large_should_not_be_valid() {
        let valid = is_valid(&sample_game(), &Subset { red: 3, green: 2, blue: 6, });
        assert!(!valid, "Game was valid!")
    }

    #[test]
    fn when_green_is_too_large_should_not_be_valid() {
        let valid = is_valid(&sample_game(), &Subset { red: 4, green: 1, blue: 6, });
        assert!(!valid, "Game was valid!")
    }

    #[test]
    fn when_blue_is_too_large_should_not_be_valid() {
        let valid = is_valid(&sample_game(), &Subset { red: 4, green: 2, blue: 5, });
        assert!(!valid, "Game was valid!")
    }

    #[test]
    fn fewest_necessary_should_return_expected_minima() {
        let minima = fewest_necessary(&sample_game());
        assert_eq!(minima, Subset { red: 4, green: 2, blue: 6, });
    }

    #[test]
    fn should_determine_the_correct_power_of_the_sample_game_min_set() {
        let power = power(&fewest_necessary(&sample_game()));
        assert_eq!(power, 48);
    }
}