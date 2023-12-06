

pub fn get_numbers<T: std::str::FromStr>(string: &str) -> Vec<T> {
    string.split(' ')
        .filter_map(|sub_string| sub_string.parse::<T>().ok())
        .collect()
}

pub fn split_first(string: &str, split_at: char) -> Option<(&str, &str)> {
    string.find(split_at)
        .map_or(None, |idx| Some((&string[..idx], &string[idx + 1..])))
}