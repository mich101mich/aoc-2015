use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{str} to {str} = {usize}").unwrap())
        .to_vec();

    let mut name_map = HashMap::new();
    let mut costs = Grid::new();
    let mut max_cost = 0;
    for (a, b, cost) in &parsed {
        for name in [a, b] {
            if !name_map.contains_key(name) {
                name_map.insert(*name, name_map.len());
                costs.extend_clone((name_map.len(), name_map.len()), 0);
            }
        }
        let a = *name_map.get(a).unwrap();
        let b = *name_map.get(b).unwrap();
        costs[(a, b)] = *cost;
        costs[(b, a)] = *cost;
        max_cost = max_cost.max(*cost);
    }

    let start_id = name_map.len();
    let goal_id = start_id + 1;
    costs.extend_by_clone((2, 2), 0);

    let start = (start_id, (1u32 << start_id));
    let goal = (goal_id, (1 << costs.w()) - 1);

    let path = a_star_search(
        |(pos, visited), out| {
            let added = costs[pos]
                .iter()
                .enumerate()
                .filter(|(i, _)| visited & (1 << i) == 0)
                .map(|(i, cost)| ((i, visited | (1 << i)), max_cost - *cost));
            out.extend(added)
        },
        start,
        goal,
        |_| 0,
    )
    .unwrap();

    let cost = (path.path.len() - 1) * max_cost - path.cost;
    pv!(cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{str} to {str} = {usize}").unwrap())
        .to_vec();

    let mut name_map = HashMap::new();
    let mut costs = Grid::new();
    for (a, b, cost) in &parsed {
        for name in [a, b] {
            if !name_map.contains_key(name) {
                name_map.insert(*name, name_map.len());
                costs.extend_clone((name_map.len(), name_map.len()), 0);
            }
        }
        let a = *name_map.get(a).unwrap();
        let b = *name_map.get(b).unwrap();
        costs[(a, b)] = *cost;
        costs[(b, a)] = *cost;
    }

    let start_id = name_map.len();
    let goal_id = start_id + 1;
    costs.extend_by_clone((2, 2), 0);

    let start = (start_id, (1u32 << start_id));
    let goal = (goal_id, (1 << costs.w()) - 1);

    let path = a_star_search(
        |(pos, visited), out| {
            let added = costs[pos]
                .iter()
                .enumerate()
                .filter(|(i, _)| visited & (1 << i) == 0)
                .map(|(i, cost)| ((i, visited | (1 << i)), *cost));
            out.extend(added)
        },
        start,
        goal,
        |_| 0,
    )
    .unwrap();

    pv!(path.cost);
}
