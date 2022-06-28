use crate::utils::*;
use std::cmp::{Ordering::*, Reverse};

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    fn count_combinations(
        buckets: &[usize],
        n: usize,
        count: usize,
        best_count: &mut (usize, usize),
    ) {
        if buckets.is_empty() {
            if n != 0 {
                return;
            }
            *best_count = match count.cmp(&best_count.0) {
                Less => (count, 1),
                Equal => (best_count.0, best_count.1 + 1),
                Greater => return,
            };
            return;
        }
        let (current, others) = buckets.split_first().unwrap();
        count_combinations(others, n, count, best_count);
        if count > best_count.0 {
            return;
        }
        match current.cmp(&n) {
            Less => count_combinations(others, n - current, count + 1, best_count),
            Equal => count_combinations(&[], 0, count + 1, best_count),
            Greater => (),
        };
    }

    let parsed = input.lines().map(parse_u).to_vec();
    let mut best_count = (usize::MAX, 0);
    count_combinations(&parsed, 150, 0, &mut best_count);
    pv!(best_count.1);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    fn count_combinations(buckets: &[usize], n: usize) -> usize {
        if buckets.is_empty() {
            return if n == 0 { 1 } else { 0 };
        }
        let (current, others) = buckets.split_first().unwrap();
        let mut combinations = count_combinations(others, n);
        combinations += match current.cmp(&n) {
            Less => count_combinations(others, n - current),
            Equal => 1,
            Greater => 0,
        };
        combinations
    }
    let parsed = input.lines().map(parse_u).to_vec();
    let count = count_combinations(&parsed, 150);
    pv!(count);
}
