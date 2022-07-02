use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let min_qe = calc_min_qe::<4>(input);
    pv!(min_qe);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let min_qe = calc_min_qe::<3>(input);
    pv!(min_qe);
}

fn calc_min_qe<const N: usize>(input: &str) -> usize {
    let mut parsed = input.lines().map(parse_u).to_vec();
    parsed.sort_unstable_by(|a, b| b.cmp(a));

    let target = parsed.iter().sum::<usize>() / N;

    let mut min_qe = usize::MAX;
    fill_first_group::<N>(&parsed, &mut vec![], 0, target, 1, &mut min_qe);
    min_qe
}

fn fill_first_group<const N: usize>(
    items: &[usize],
    remaining: &mut Vec<usize>,
    current: usize,
    target: usize,
    qe: usize,
    min_qe: &mut usize,
) {
    if items.is_empty() {
        return;
    }
    let start_len = remaining.len();
    let (&item, items) = items.split_first().unwrap();
    let with_qe = qe * item;
    if with_qe < *min_qe {
        use std::cmp::Ordering::*;
        match target.cmp(&(current + item)) {
            Less => (), // doesn't fit
            Equal => {
                remaining.extend(items);
                let mut counts = [target; N];
                counts[0] = 0;
                if can_fill_remaining_groups(remaining, counts) {
                    *min_qe = with_qe;
                }
                remaining.truncate(start_len);
            }
            Greater => {
                fill_first_group::<N>(items, remaining, current + item, target, with_qe, min_qe);
            }
        }
    }
    if qe >= *min_qe {
        return;
    }
    remaining.push(item);
    fill_first_group::<N>(items, remaining, current, target, qe, min_qe);
    remaining.pop();
}

fn can_fill_remaining_groups<const N: usize>(items: &[usize], counts: [usize; N]) -> bool {
    if items.is_empty() {
        assert_eq!(counts, [0; N]);
        return true;
    }
    let (&item, items) = items.split_first().unwrap();
    for i in 1..N {
        if counts[i] < item {
            continue;
        }
        let mut counts = counts;
        counts[i] -= item;
        if items.is_empty() {
            assert_eq!(counts, [0; N]);
            return true;
        }
        if can_fill_remaining_groups(items, counts) {
            return true;
        }
    }
    false
}
