use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let mut floor = 0;
    let parsed = input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .position(|i| {
            floor += i;
            floor == -1
        })
        .unwrap()
        + 1;

    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let parsed = input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .sum::<i32>();

    pv!(parsed);
}
