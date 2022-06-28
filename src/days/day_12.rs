use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    fn sum_numbers(input: &json::JsonValue) -> isize {
        use json::JsonValue::*;
        match input {
            Number(n) => isize::try_from(*n).ok().unwrap(),
            Array(a) => a.iter().map(sum_numbers).sum(),
            Object(o) => {
                if o.iter().any(|(_, v)| v == "red") {
                    0
                } else {
                    o.iter().map(|(_, v)| sum_numbers(v)).sum()
                }
            }
            _ => 0,
        }
    }

    let parsed = json::parse(input).unwrap();
    let sum = sum_numbers(&parsed);
    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    fn sum_numbers(input: &json::JsonValue) -> isize {
        use json::JsonValue::*;
        match input {
            Number(n) => isize::try_from(*n).ok().unwrap(),
            Array(a) => a.iter().map(sum_numbers).sum(),
            Object(o) => o.iter().map(|(_, v)| sum_numbers(v)).sum(),
            _ => 0,
        }
    }

    let parsed = json::parse(input).unwrap();
    let sum = sum_numbers(&parsed);
    pv!(sum);
}
