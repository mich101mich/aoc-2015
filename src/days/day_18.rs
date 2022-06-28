use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let mut grid = hashtag_grid(input);
    let mut next_grid = grid.clone();

    let neighborhood = grid.moore();
    let last = grid.w() - 1;

    for _ in 0..100 {
        for (pos, v) in next_grid.grid_iter_mut_index() {
            if (pos.0 == 0 || pos.0 == last) && (pos.1 == 0 || pos.1 == last) {
                *v = true;
                continue;
            }
            let count = neighborhood
                .get_all_neighbors(pos)
                .filter(|p| grid[p])
                .count();
            *v = matches!((grid[pos], count), (true, 2 | 3) | (false, 3));
        }
        std::mem::swap(&mut grid, &mut next_grid);
    }
    let count = grid.count();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let mut grid = hashtag_grid(input);
    let mut next_grid = grid.clone();

    let neighborhood = grid.moore();

    for _ in 0..100 {
        for (pos, v) in next_grid.grid_iter_mut_index() {
            let count = neighborhood
                .get_all_neighbors(pos)
                .filter(|p| grid[p])
                .count();
            *v = matches!((grid[pos], count), (true, 2 | 3) | (false, 3));
        }
        std::mem::swap(&mut grid, &mut next_grid);
    }
    let count = grid.count();
    pv!(count);
}
