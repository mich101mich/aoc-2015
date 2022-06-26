use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let parsed = input
        .lines()
        .map(|l| l.chars().to_vec())
        .filter(|l| {
            let mut pairs = HashMap::<&[char], Vec<usize>>::new();
            let mut found = false;
            for (i, c) in l.windows(2).enumerate() {
                if let Some(pos) = pairs.get(c) {
                    if pos.iter().any(|p| diff(*p, i) > 1) {
                        found = true;
                        break;
                    }
                }
                pairs.entry(c).or_insert_with(Vec::new).push(i);
            }
            found && l.windows(3).any(|w| w[0] == w[2])
        })
        .count();
    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let vowels = "aeiou".chars().to_set();
    let bad = [['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y']];
    let bad = bad.iter().map(|x| &x[..]).to_set();

    let parsed = input
        .lines()
        .map(|l| l.chars().to_vec())
        .filter(|l| {
            l.iter().filter(|c| vowels.contains(c)).count() >= 3
                && l.windows(2).any(|c| c[0] == c[1])
                && !l.windows(2).any(|c| bad.contains(&c))
        })
        .count();
    pv!(parsed);
}
