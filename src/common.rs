use num::Num;

pub fn get_numbers<T: std::str::FromStr>(string: &str) -> Vec<T> {
    string.split(' ')
        .filter_map(|sub_string| sub_string.parse::<T>().ok())
        .collect()
}

pub fn split_first(string: &str, split_at: char) -> Option<(&str, &str)> {
    string.find(split_at)
        .map_or(None, |idx| Some((&string[..idx], &string[idx + 1..])))
}

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}


#[derive(Clone, Copy, Debug)]
pub enum Direction { North, East, South, West }

impl Direction {
    pub fn as_power_of_2(&self) -> usize {
        match *self {
            Direction::North => 0b0001,
            Direction::East => 0b0010,
            Direction::South => 0b0100,
            Direction::West => 0b1000,
        }
    }
}

pub fn next_coord<T: Num + PartialOrd + Clone>((x, y): (T, T), direction: Direction, (width, height): (T, T)) -> Option<(T, T)> {
    match direction {
        Direction::West => if x > T::zero() { Some((x - T::one(), y)) } else { None },
        Direction::East => if x < width - T::one() { Some((x + T::one(), y)) } else { None },
        Direction::North => if y > T::zero() { Some((x, y - T::one())) } else { None },
        Direction::South => if y < height - T::one() { Some((x, y + T::one())) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{gcd, lcm};

    #[test]
    fn should_correctly_compute_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 48), 6);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(8, 12), 4);
        assert_eq!(gcd(49, 13), 1);
    }

    #[test]
    fn should_correctly_compute_lcm() {
        assert_eq!(lcm(48, 18), 144);
        assert_eq!(lcm(18, 48), 144);
        assert_eq!(lcm(54, 24), 216);
        assert_eq!(lcm(8, 12), 24);
        assert_eq!(lcm(49, 13), 49 * 13);
    }
}
