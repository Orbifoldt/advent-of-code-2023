use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::fs;
use std::iter::zip;

use crate::common::split_first;
use crate::day07::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

pub fn main() {
    let input = &fs::read_to_string("./day07/input.txt").unwrap();
    part1(input);
    part2(input);
}

pub fn part1(input: &str) -> i64 {
    let mut hands = input.lines().map(|line| Hand::new(line)).collect::<Vec<_>>();
    hands.sort();
    let winnings = hands.iter().enumerate().map(|(rank, hand)| {
        let rank = rank as i64 + 1;
        println!("Rank {rank}: {:?}", hand);
        rank * hand.bid
    }).sum();
    println!("Total winnings for part 1: {winnings}");
    winnings
}

pub fn part2(input: &str) -> i64 {
    let mut hands = input.lines().map(|line| Hand::new_pt2(line)).collect::<Vec<_>>();
    hands.sort_by(|a,b | {
        cmp_hands_pt2(a, b)
    });
    let winnings = hands.iter().enumerate().map(|(rank, hand)| {
        let rank = rank as i64 + 1;
        println!("Rank {rank}: {:?}", hand);
        rank * hand.bid
    }).sum();
    println!("Total winnings for part 1: {winnings}");
    winnings
}

#[derive(Eq, PartialEq, Debug)]
pub struct Hand {
    cards: [char; 5],
    bid: i64,
    hand_type: HandType,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}


impl Hand {
    pub fn new(input: &str) -> Self {
        let (cards_string, bid_string) = split_first(input, ' ').unwrap();
        let cards = cards_string.chars().take(5).collect::<Vec<_>>().try_into().unwrap();
        Self {
            cards: cards,
            bid: bid_string.parse().expect("Bid should be a valid number"),
            hand_type: determine_type(&cards),
        }
    }

    pub fn new_pt2(input: &str) -> Self {
        let (cards_string, bid_string) = split_first(input, ' ').unwrap();
        let cards = cards_string.chars().take(5).collect::<Vec<_>>().try_into().unwrap();
        Self {
            cards: cards,
            bid: bid_string.parse().expect("Bid should be a valid number"),
            hand_type: determine_type_pt2(&cards),
        }
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_cmp = self.hand_type.cmp(&other.hand_type).reverse();
        match type_cmp {
            Equal => cmp_char_arrays(&self.cards, &other.cards, cmp_chars),
            _ => type_cmp,
        }
    }
}

fn cmp_chars(a: &char, b: &char) -> Ordering {
    if a == b { return Equal; }

    if a.is_numeric() {
        return if b.is_numeric() { a.cmp(b) /* comparing chars, not numbers */ } else { Less };
    }

    if b.is_numeric() /* && !a.is_numeric() */ {
        return Greater;
    }

    // Botha a and b alphabetic
    let letter_order = ['A', 'K', 'Q', 'J', 'T'];
    let idx_a = letter_order.iter().position(|c| *c == *a).unwrap();
    let idx_b = letter_order.iter().position(|c| *c == *b).unwrap();
    return idx_a.cmp(&idx_b).reverse();
}

fn cmp_char_arrays(a: &[char; 5], b: &[char; 5], char_comparator: fn(&char, &char) -> Ordering) -> Ordering {
    zip(a, b).filter_map(|(ai, bi)| {
        if ai != bi {
            Some(char_comparator(ai, bi))
        } else {
            None
        }
    }).next().unwrap_or(Equal)
}

fn determine_type(cards: &[char; 5]) -> HandType {
    let frequencies = cards.iter().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    if frequencies.values().any(|f| *f == 5) {
        FiveOfAKind
    } else if frequencies.values().any(|f| *f == 4) {
        FourOfAKind
    } else if frequencies.values().any(|f| *f == 3) && frequencies.values().any(|f| *f == 2) {
        FullHouse
    } else if frequencies.values().any(|f| *f == 3) {
        ThreeOfAKind
    } else if frequencies.values().filter(|&f| *f == 2).count() == 2 {
        TwoPair
    } else if frequencies.values().any(|f| *f == 2) {
        OnePair
    } else {
        HighCard
    }
}

fn determine_type_pt2(cards: &[char; 5]) -> HandType {
    let frequencies = cards.iter().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    if frequencies.get(&'J').is_none() {
        println!("\nHand {:?} doesn't contain any Jokers", cards);
        return determine_type(cards);
    }

    println!("\nHand {:?} contains Jokers, determining best use of them...", cards);
    let mut cards_to_try = frequencies.keys().filter(|c| ***c != 'J').collect::<Vec<_>>();
    if cards_to_try.is_empty() {
        cards_to_try = vec![&&'A']
    }

    let perms = cards.iter().enumerate()
        .fold(vec![vec![]], |acc, (idx, c)| {
            let nxt_gen = acc.iter().flat_map(|mut sub_str: &Vec<char>| {
                let extra_stuff: Vec<Vec<char>> = if *c != 'J' {
                    let mut clone = sub_str.clone();
                    clone.push(*c);

                    vec![clone]
                } else {
                    println!("Seeing how we can replace {c} at idx {idx}");
                    cards_to_try.iter().map(|replacement| {
                        println!(" - trying to replace joker with {replacement}");
                        let mut clone = sub_str.clone();
                        clone.push(***replacement);
                        clone
                    }).collect()
                };
                extra_stuff
            }).collect::<Vec<Vec<char>>>();
            nxt_gen
        });
    println!("All possible permutations for {:?}:\n  > {:?}",cards, perms);
    let mut possibilities = perms.iter().map(|perm| {
        let arr: &[char] = perm.as_slice();
        if arr.len() != 5 { panic!() }
        let arr: &[char; 5] = <&[char; 5]>::try_from(arr).unwrap();
        (arr, determine_type(arr))
    }).collect::<Vec<_>>();
    possibilities.sort_by(|a, b| a.1.cmp(&b.1));
    let (a, b) = possibilities.first().unwrap();
    println!("Best use of jokers is {:?} which gives type {:?}", a, b);
    *b
}

fn cmp_hands_pt2(a: &Hand, b: &Hand) -> Ordering {
    let type_cmp = a.hand_type.cmp(&b.hand_type).reverse();
    match type_cmp {
        Equal => cmp_char_arrays(&a.cards, &b.cards, cmp_chars_pt2),
        _ => type_cmp,
    }
}

fn cmp_chars_pt2(a: &char, b: &char) -> Ordering {
    if a == b { return Equal; }

    let letter_order = ['A', 'K', 'Q', 'T','9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    let idx_a = letter_order.iter().position(|c| *c == *a).unwrap();
    let idx_b = letter_order.iter().position(|c| *c == *b).unwrap();
    return idx_a.cmp(&idx_b).reverse();
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering::{Greater, Less};
    use std::fs;

    use crate::day07::{Hand, part1, part2};
    use crate::day07::HandType::TwoPair;

    #[test]
    fn should_correctly_parse_string_into_hand() {
        let hand = Hand::new("KTJJT 220");
        assert_eq!(hand, Hand {
            cards: ['K', 'T', 'J', 'J', 'T'],
            bid: 220,
            hand_type: TwoPair,
        });
    }

    #[test]
    fn should_correctly_compare_hands() {
        let hands = fs::read_to_string("./day07/input_example.txt").unwrap().lines().map(|line|
            Hand::new(line)
        ).collect::<Vec<_>>();
        let [h1, h4, h3, h2, h5] = hands.try_into().unwrap();
        // QQQJA is largest three of a kind
        assert_eq!(h5.cmp(&h1), Greater);
        assert_eq!(h5.cmp(&h2), Greater);
        assert_eq!(h5.cmp(&h3), Greater);
        assert_eq!(h5.cmp(&h4), Greater);
        // Then T55J5
        assert_eq!(h4.cmp(&h1), Greater);
        assert_eq!(h4.cmp(&h2), Greater);
        assert_eq!(h4.cmp(&h3), Greater);
        // Then KK677 is largest two pair
        assert_eq!(h3.cmp(&h1), Greater);
        assert_eq!(h3.cmp(&h2), Greater);
        // Then KTJJT
        assert_eq!(h2.cmp(&h1), Greater);

        //  32T3K is one pair and smallest of all
        assert_eq!(h1.cmp(&h2), Less);
        assert_eq!(h1.cmp(&h3), Less);
        assert_eq!(h1.cmp(&h4), Less);
        assert_eq!(h1.cmp(&h5), Less);
    }

    #[test]
    fn bugfix__should_correctly_compare_numbers() {
        let hand1 = Hand::new("A9AAA 220");
        let hand2 = Hand::new("A8AAA 220");

        assert_eq!(hand1.cmp(&hand2), Greater)
    }

    #[test]
    fn should_return_correct_total_winnings() {
        let x = part1(&fs::read_to_string("./day07/input_example.txt").unwrap());
        assert_eq!(x, 6440)
    }

    #[test]
    fn should_return_correct_total_winnings_part2() {
        let x = part2(&fs::read_to_string("./day07/input_example.txt").unwrap());
        assert_eq!(x, 5905)
    }
}