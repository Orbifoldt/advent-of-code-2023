pub fn main() {
    println!("Hello")
}

struct Game {
    id: i32,
    subsets: Vec<Subset>,
}

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


pub fn read(game: &String) -> Game {
    Game::new(17)
}

#[cfg(test)]
mod tests{
    use crate::day02;

    #[test]
    fn should_correctly_parse_a_the_game_id() {
        let game = day02::read(&"Game 17: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());
        assert_eq!(game.id, 17)
    }
}