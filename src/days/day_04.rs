use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    for i in 1.. {
        let hash = md5::compute(&format!("{}{}", input, i));
        if format!("{hash:x}").starts_with("000000") {
            pv!(i);
            break;
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    for i in 1.. {
        let hash = md5::compute(&format!("{}{}", input, i));
        if format!("{hash:x}").starts_with("00000") {
            pv!(i);
            break;
        }
    }
}
