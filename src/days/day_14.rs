use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let time = 2503;

    let reindeer = input
        .lines()
        .map(|l| {
            scanf!(
        l,
        "{str} can fly {usize} km/s for {usize} seconds, but then must rest for {usize} seconds."
    )
            .unwrap()
        })
        .to_vec();

    fn lead(reindeer: &[(&str, usize, usize, usize)], time: usize) -> usize {
        let mut max_i = 0;
        let mut max_distance = 0;

        for (i, (_, speed, duration, rest)) in reindeer.iter().copied().enumerate() {
            let cycle_length = duration + rest;
            let distance = time / cycle_length * speed * duration
                + (time % cycle_length).min(duration) * speed;

            if distance > max_distance {
                max_i = i;
                max_distance = distance;
            }
        }
        max_i
    }

    let mut scores = vec![0; reindeer.len()];
    for i in 1..=time {
        scores[lead(&reindeer, i)] += 1;
    }
    let max = scores.iter().max().unwrap();
    pv!(max);
    // 1060 high
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let time = 2503;
    let mut max_distance = 0;

    for l in input.lines() {
        let (name, speed, duration, rest) = scanf!(
                l,
                "{str} can fly {usize} km/s for {usize} seconds, but then must rest for {usize} seconds."
            )
            .unwrap();

        let cycle_length = duration + rest;
        let mut distance = 0;
        let full_cycles = time / cycle_length;
        distance += full_cycles * duration * speed;

        let remaining_time = time % cycle_length;
        distance += speed * remaining_time.min(duration);

        max_distance = max_distance.max(distance);
    }
    pv!(max_distance);
}
