use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{str}: capacity {isize}, durability {isize}, flavor {isize}, texture {isize}, calories {usize}").unwrap())
        .map(|(_, c, d, f, t, cal)| ([c, d, f, t], cal))
        .to_vec();

    fn get_max_score(
        ingredients: &[([isize; 4], usize)],
        totals: [isize; 4],
        calories: usize,
        teaspoons: usize,
    ) -> usize {
        for (i, t) in totals.iter().enumerate() {
            if *t <= 0 && ingredients.iter().all(|(ing, _)| ing[i] <= 0) {
                // a total <= 0 makes the whole product 0, so any remaining ingredient would have to increase that value.
                return 0;
            }
        }

        let ((current, cur_cals), others) = ingredients.split_first().unwrap();
        if others.is_empty() {
            let amount = calories / cur_cals;
            if calories % cur_cals != 0 || amount != teaspoons {
                return 0;
            }
            let mut totals = totals;
            return totals
                .iter()
                .zip(current)
                .map(|(t, c)| *t + c * amount as isize)
                .map(|x| x.max(0) as usize)
                .product();
        }

        let max_remaining = (calories / cur_cals).min(teaspoons);

        let mut max_score = 0;
        for amount in 0..=max_remaining {
            let mut totals = totals;
            let calories = calories - cur_cals * amount;
            let teaspoons = teaspoons - amount;
            for (t, c) in totals.iter_mut().zip(current) {
                *t += c * amount as isize;
            }
            let score = get_max_score(others, totals, calories, teaspoons);
            max_score = max_score.max(score);
        }
        max_score
    }
    let max_score = get_max_score(&parsed, [0, 0, 0, 0], 500, 100);
    pv!(max_score);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{str}: capacity {isize}, durability {isize}, flavor {isize}, texture {isize}, calories {isize}").unwrap())
        .map(|(_, c, d, f, t, _)| [c, d, f, t])
        .to_vec();

    fn get_max_score(ingredients: &[[isize; 4]], totals: [isize; 4], remaining: isize) -> isize {
        if ingredients.is_empty() {
            return totals.iter().map(|x| x.max(&0)).product();
        }
        for (i, t) in totals.iter().enumerate() {
            if *t <= 0 && ingredients.iter().all(|ing| ing[i] <= 0) {
                // a total <= 0 makes the whole product 0, so any remaining ingredient would have to increase that value.
                return 0;
            }
        }
        let (current, others) = ingredients.split_first().unwrap();
        let mut max_score = 0;
        for amount in 0..=remaining {
            let mut totals = totals;
            for (t, c) in totals.iter_mut().zip(current) {
                *t += c * amount;
            }
            let score = get_max_score(others, totals, remaining - amount);
            max_score = max_score.max(score);
        }
        max_score
    }
    let max_score = get_max_score(&parsed, [0, 0, 0, 0], 100);
    pv!(max_score);
}
