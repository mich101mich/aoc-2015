use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let mut lines = input.lines();
    let mut inverse_rules = HashMap::new();
    for (a, b) in lines
        .by_ref()
        .map_while(|l| scanf!(l, "{str} => {str}").ok())
    {
        if inverse_rules.contains_key(&b) {
            panic!("Duplicate inverse rule for {}", b);
        }
        inverse_rules.insert(b, a);
    }
    let mut inverse_rules = inverse_rules.into_iter().to_vec();

    let molecule = String::from(lines.last().unwrap());
    let goal = "e";

    // Approach: Good old Monte Carlo.
    // trust me, both BFS, DFS and Iterative Deepening take several minutes,
    // this takes 200ms at most.

    let mut rng = rand::thread_rng();
    loop {
        let mut molecule = molecule.clone();
        for step in 1.. {
            let mut possible = vec![];
            for (i, (target, replacer)) in inverse_rules.iter().enumerate() {
                let mut offset = 0;
                while let Some(pos) = molecule[offset..].find(target) {
                    offset += pos;
                    possible.push((i, offset));
                    offset += 1; // account for self-overlapping patterns
                }
            }
            if possible.is_empty() {
                break;
            }
            let (i, offset) = *possible.choose(&mut rng).unwrap();
            let (target, replacer) = inverse_rules[i];
            let start = &molecule[..offset];
            let end = &molecule[offset + target.len()..];
            molecule = format!("{}{}{}", start, replacer, end);
            if molecule == goal {
                pv!(step);
                return;
            }
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let mut lines = input.lines();
    let mut rules = HashMap::new();
    for (a, b) in lines
        .by_ref()
        .map_while(|l| scanf!(l, "{str} => {str}").ok())
    {
        rules.entry(a).or_insert_with(Vec::new).push(b);
    }

    let molecule = lines.last().unwrap();
    let mut possibilities = HashSet::new();
    for (target, replacers) in &rules {
        let mut offset = 0;
        while let Some(pos) = molecule[offset..].find(target) {
            offset += pos;
            let start = &molecule[..offset];
            offset += target.len();
            let end = &molecule[offset..];
            for s in replacers {
                possibilities.insert(format!("{}{}{}", start, s, end));
            }
        }
    }
    pv!(possibilities.len());
}
