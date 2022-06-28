use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    enum Reading {
        Exact(usize),
        Range(Option<usize>, Option<usize>),
    }
    use Reading::*;
    let gift = [
        ("children", Exact(3)),
        ("cats", Range(Some(7), None)),
        ("samoyeds", Exact(2)),
        ("pomeranians", Range(None, Some(3))),
        ("akitas", Exact(0)),
        ("vizslas", Exact(0)),
        ("goldfish", Range(None, Some(5))),
        ("trees", Range(Some(3), None)),
        ("cars", Exact(2)),
        ("perfumes", Exact(1)),
    ]
    .into_iter()
    .to_map();

    'outer: for l in input.lines() {
        let (n, owned) = scanf!(l, "Sue {usize}: {str}").unwrap();
        for s in owned.split(", ") {
            let (k, v) = scanf!(s, "{str}: {usize}").unwrap();
            let matched = match gift[k] {
                Exact(n) => n == v,
                Range(min, max) => {
                    min.map(|m| m < v).unwrap_or(true) && max.map(|m| m > v).unwrap_or(true)
                }
            };
            if !matched {
                continue 'outer;
            }
        }
        pv!(n);
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let gift = [
        ("children", 3usize),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .to_map();

    'outer: for l in input.lines() {
        let (n, owned) = scanf!(l, "Sue {usize}: {str}").unwrap();
        for s in owned.split(", ") {
            let (k, v) = scanf!(s, "{str}: {usize}").unwrap();
            if gift.get(&k).unwrap() != &v {
                continue 'outer;
            }
        }
        pv!(n);
    }
}
