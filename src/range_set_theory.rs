use std::ops::Range;

// Set theoretic `A \cap B` for intervals A and B
pub fn intersect_range(a: &Range<i64>, b: &Range<i64>) -> Range<i64> {
    if a.contains(&b.start) && a.contains(&b.end) {
        b.start..b.end
    } else if a.contains(&b.start) {
        b.start..a.end
    } else if a.contains(&b.end) {
        a.start..b.end
    } else if b.contains(&a.start) && b.contains(&a.end) {
        a.start..a.end
    } else {
        0..0
    }
}

// Set theoretic `\cap_i A_i` for intervals A_i
pub fn intersect_many_ranges(aa: Vec<Range<i64>>) -> Range<i64> {
    let first = aa.first().expect("Should provide at least 1 input");
    aa.iter().skip(1).fold(first.start..first.end, |acc, next| {
        intersect_range(&acc, next)
    })
}

// Set theoretic `(\cup_i A_i) \cap (\cup_j B_j)` for intervals A_i and B_j
pub fn intersect(aa: &Vec<Range<i64>>, bb: &Vec<Range<i64>>) -> Vec<Range<i64>> {
    aa.iter().flat_map(|a| {
        bb.iter().map(|b| {
            intersect_range(a, b)
        })
    })
        .filter(|rng| !rng.is_empty())
        .collect()
}

// Set theoretic `A \setminus B` for intervals A and B
pub fn cut_out(a: &Range<i64>, b: &Range<i64>) -> Vec<Range<i64>> {
    if a.contains(&b.start) && a.contains(&b.end) {
        vec![a.start..b.start, b.end..a.end]
    } else if a.contains(&b.start) {
        vec![a.start..b.start]
    } else if a.contains(&b.end) {
        vec![b.end..a.end]
    } else if b.contains(&a.start) && b.contains(&a.end) {
        vec![]
    } else {
        vec![a.start..a.end]
    }
}

// Set theoretic `A \cup B` for intervals A and B
pub fn union(a: &Range<i64>, b: &Range<i64>) -> Vec<Range<i64>> {
    if a.contains(&b.start) && a.contains(&b.end) {
        vec![a.start..a.end]
    } else if a.contains(&b.start) {
        vec![a.start..b.end]
    } else if a.contains(&b.end) {
        vec![b.start..a.end]
    } else if b.contains(&a.start) && b.contains(&a.end) {
        vec![b.start..b.end]
    } else {
        if a.end==b.start {
            vec![a.start..b.end]
        } else if b.end == a.start {
            vec![b.start..a.end]
        } else {
            vec![a.start..a.end, b.start..b.end]
        }
    }
}

// Set theoretic `\cup_i A_i` for intervals A_i
pub fn union_many(aa: &Vec<Range<i64>>) -> Vec<Range<i64>> {
    aa.iter().fold(vec![0..0], |union_vec, a| {
        union_vec.iter()
            .flat_map(|a_prime| union(a, a_prime))
            .filter(|rng| !rng.is_empty())
            .collect()
    })
}


// Set theoretic `A \setminus (\cup_i B_i) = \cap_i (A \setminus B_i)` for intervals A and B_i
pub fn cut_out_many(a: Range<i64>, bs: &Vec<Range<i64>>) -> Vec<Range<i64>> {
    let a_without_bs: Vec<Vec<Range<i64>>> = bs.iter()
        .map(|b| cut_out(&a, b))
        .collect();

    let first = a_without_bs.iter().next().unwrap().clone();
    a_without_bs.iter().fold(first, |c, d| {
        intersect(&c, d)
    })
}

#[cfg(test)]
mod tests {
    use crate::range_set_theory::{cut_out_many, intersect, intersect_many_ranges, intersect_range, union, union_many};

    #[test]
    fn should_correctly_intersect_ranges() {
        assert_eq!(intersect_range(&(0..4), &(2..6)), 2..4);
        assert!(intersect_range(&(0..4), &(4..6)).is_empty());
        assert_eq!(intersect_range(&(2..6), &(0..4)), 2..4);
        assert_eq!(intersect_range(&(0..4), &(1..3)), 1..3);
    }

    #[test]
    fn should_correctly_intersect_many_ranges() {
        assert_eq!(intersect_many_ranges(vec![(2..10), (0..8), (4..9)]), 4..8);
    }

    #[test]
    fn should_correctly_intersect_range_unions() {
        assert_eq!(intersect(
            &vec![2..5, 10..15, 20..25],
            &vec![4..12, 14..18, 23..28],
        ),
                   vec![4..5, 10..12, 14..15, 23..25]);
    }

    #[test]
    fn should_correctly_union_two_ranges() {
        assert_eq!(union(&(1..3), &(4..6) ), vec![1..3, 4..6]);
        assert_eq!(union(&(1..4), &(3..6) ), vec![1..6]);
        assert_eq!(union(&(1..3), &(3..6) ), vec![1..6]);
        assert_eq!(union(&(3..6), &(1..3) ), vec![1..6]);
    }

    #[test]
    fn should_correctly_union_many_ranges() {
        assert_eq!(union_many(&vec![1..3, 4..6] ), vec![4..6, 1..3]);
        assert_eq!(union_many(&vec![1..4, 3..6] ), vec![1..6]);
        assert_eq!(union_many(&vec![1..3, 3..6] ), vec![1..6]);
        assert_eq!(union_many(&vec![3..6, 1..3] ), vec![1..6]);

        let union = union_many(&vec![6..10, 4..6, 7..8, 1..3, 4..6] );
        assert!(vec![1,2,4,5,6,7,8,9].iter().all(|x| union.iter().any(|rng| rng.contains(x))));
    }

    #[test]
    fn should_correctly_cut_out_many_unions() {
        assert_eq!(cut_out_many(
            10..50,
            &vec![1..5, 8..12, 20..30, 40..44, 42..45, 48..57],
        ),
                   vec![12..20, 30..40, 45..48]);
    }
}