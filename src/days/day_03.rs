use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let mut visited = HashSet::new();
    let mut santa = (0isize, 0isize);
    let mut robot = santa;
    visited.insert(santa);

    for (i, d) in input
        .chars()
        .map(Dir::from_char)
        .map(Result::unwrap)
        .enumerate()
    {
        if i % 2 == 0 {
            santa += d;
            visited.insert(santa);
        } else {
            robot += d;
            visited.insert(robot);
        }
    }
    pv!(visited.len());
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let mut visited = HashSet::new();
    let mut pos = (0isize, 0isize);
    visited.insert(pos);
    for d in input.chars().map(Dir::from_char).map(Result::unwrap) {
        pos += d;
        visited.insert(pos);
    }
    pv!(visited.len());
}
