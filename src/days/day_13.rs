use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let mut name_map = HashMap::new();
    let mut next_id = 0usize..;
    let mut relations = Grid::new();

    let iter = input.lines().chain(std::iter::once(
        "___ would gain 0 happiness units by sitting next to ___.",
    ));

    for l in iter {
        let (a, gain, amount, b) = scanf!(
            l,
            "{str} would {str} {isize} happiness units by sitting next to {str}."
        )
        .unwrap();
        let amount = if gain == "gain" { amount } else { -amount };
        let a = *name_map.entry(a).or_insert_with(|| next_id.next().unwrap());
        let b = *name_map.entry(b).or_insert_with(|| next_id.next().unwrap());
        if relations.len() < name_map.len() {
            relations.extend_clone((name_map.len(), name_map.len()), 0isize);
        }
        relations[a][b] = amount;
    }

    let n = name_map.len();
    let loss_shift = relations.grid_iter().map(|x| -x).max().unwrap();
    let max_cost = relations
        .grid_iter()
        .map(|x| (x + loss_shift) as usize)
        .max()
        .unwrap();
    let relations = relations.map(|x| max_cost - (x + loss_shift) as usize);

    let start = (0usize, 1u32); // first person seated
    let goal = (n, (1 << n) - 1); // everyone seated

    let path = a_star_search(
        |(prev, seated), out| {
            for i in 1..n {
                if seated & (1 << i) != 0 {
                    continue;
                }
                let mut cost = relations[prev][i] + relations[i][prev];
                let mut target = i;
                let seated = seated | (1 << i);
                if seated == goal.1 {
                    cost += relations[i][0] + relations[0][i];
                    out.push((goal, cost));
                } else {
                    out.push(((i, seated), cost));
                }
            }
        },
        start,
        goal,
        |_| 0,
    )
    .unwrap();

    let cost = max_cost * 2 * n - path.cost;
    let cost = cost as isize - loss_shift * 2 * n as isize;
    pv!(cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let mut name_map = HashMap::new();
    let mut next_id = 0usize..;
    let mut relations = Grid::new();

    for l in input.lines() {
        let (a, gain, amount, b) = scanf!(
            l,
            "{str} would {str} {isize} happiness units by sitting next to {str}."
        )
        .unwrap();
        let amount = if gain == "gain" { amount } else { -amount };
        let a = *name_map.entry(a).or_insert_with(|| next_id.next().unwrap());
        let b = *name_map.entry(b).or_insert_with(|| next_id.next().unwrap());
        if relations.len() < name_map.len() {
            relations.extend_clone((name_map.len(), name_map.len()), 0isize);
        }
        relations[a][b] = amount;
    }

    let n = name_map.len();
    let loss_shift = relations.grid_iter().map(|x| -x).max().unwrap();
    let max_cost = relations
        .grid_iter()
        .map(|x| (x + loss_shift) as usize)
        .max()
        .unwrap();
    let relations = relations.map(|x| max_cost - (x + loss_shift) as usize);

    let start = (0usize, 1u32); // first person seated
    let goal = (n, (1 << n) - 1); // everyone seated

    let path = a_star_search(
        |(prev, seated), out| {
            for i in 1..n {
                if seated & (1 << i) != 0 {
                    continue;
                }
                let mut cost = relations[prev][i] + relations[i][prev];
                let mut target = i;
                let seated = seated | (1 << i);
                if seated == goal.1 {
                    cost += relations[i][0] + relations[0][i];
                    out.push((goal, cost));
                } else {
                    out.push(((i, seated), cost));
                }
            }
        },
        start,
        goal,
        |_| 0,
    )
    .unwrap();

    let cost = max_cost * 2 * n - path.cost;
    let cost = cost as isize - loss_shift * 2 * n as isize;
    pv!(cost);
}
