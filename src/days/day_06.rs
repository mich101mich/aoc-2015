use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let mut grid = Grid::new_clone((1000, 1000), 0usize);

    for l in input.lines() {
        let (instr, x0, y0, x1, y1) =
            scanf!(l, "{str} {usize},{usize} through {usize},{usize}").unwrap();

        if instr == "turn on" {
            for y in y0..=y1 {
                grid.row_mut(y)
                    .skip(x0)
                    .take(x1 - x0 + 1)
                    .for_each(|x| *x += 1);
            }
        } else if instr == "turn off" {
            for y in y0..=y1 {
                grid.row_mut(y)
                    .skip(x0)
                    .take(x1 - x0 + 1)
                    .for_each(|x| *x = x.saturating_sub(1));
            }
        } else if instr == "toggle" {
            for y in y0..=y1 {
                grid.row_mut(y)
                    .skip(x0)
                    .take(x1 - x0 + 1)
                    .for_each(|x| *x += 2);
            }
        }
    }
    let power = grid.sum();
    pv!(power);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let mut grid = Grid::new_clone((1000, 1000), false);

    for l in input.lines() {
        let (instr, x0, y0, x1, y1) =
            scanf!(l, "{str} {usize},{usize} through {usize},{usize}").unwrap();

        if instr == "turn on" {
            for y in y0..=y1 {
                grid.row_mut(y)
                    .skip(x0)
                    .take(x1 - x0 + 1)
                    .for_each(|x| *x = true);
            }
        } else if instr == "turn off" {
            for y in y0..=y1 {
                grid.row_mut(y)
                    .skip(x0)
                    .take(x1 - x0 + 1)
                    .for_each(|x| *x = false);
            }
        } else if instr == "toggle" {
            for y in y0..=y1 {
                grid.row_mut(y)
                    .skip(x0)
                    .take(x1 - x0 + 1)
                    .for_each(|x| *x = !*x);
            }
        }
    }
    let lit = grid.count();
    pv!(lit);
}
