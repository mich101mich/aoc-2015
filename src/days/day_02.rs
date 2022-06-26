use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{usize}x{usize}x{usize}").unwrap())
        .map(|(l, w, h)| {
            let sides = [(l, w), (w, h), (h, l)];
            let ribbon = sides.iter().map(|(a, b)| 2 * (a + b)).min().unwrap();
            let bow = l * w * h;
            ribbon + bow
        })
        .sum::<usize>();

    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{usize}x{usize}x{usize}").unwrap())
        .map(|(l, w, h)| {
            let sides = [(l, w), (w, h), (h, l)];
            let sides = sides.iter().map(|(a, b)| a * b);
            sides.clone().map(|s| s * 2).sum::<usize>() + sides.min().unwrap()
        })
        .sum::<usize>();

    pv!(parsed);
}
