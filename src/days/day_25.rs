use crate::utils::*;
use num::BigUint;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/25.txt");
    let (row, column) = scanf!(input, "To continue, please consult the code grid in the manual.  Enter the code at row {usize}, column {usize}.").unwrap();

    let target_row = row + column - 1;
    let left_value = 1 + target_row * (target_row - 1) / 2;
    let target_offset = left_value + column - 1;

    let mut value: BigUint = 20151125usize.into();
    let factor: BigUint = 252533usize.into();
    let exponent: BigUint = (target_offset - 1).into();
    let modulo: BigUint = 33554393usize.into();

    value = (value * factor.modpow(&exponent, &modulo)) % modulo;
    pv!(value);
}
