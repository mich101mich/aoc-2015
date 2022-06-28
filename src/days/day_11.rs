use crate::utils::*;

fn increment(pw: &mut [u8; 8]) {
    for v in pw.iter_mut().rev() {
        if *v == b'z' {
            *v = b'a';
        } else {
            *v += 1;
            while matches!(*v, b'i' | b'o' | b'l') {
                // if it was invalid, it would just continue counting until it reaches this point again.
                *v += 1;
            }
            break;
        }
    }
}

fn is_valid(pw: &[u8; 8]) -> bool {
    pw.windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
        && pw.windows(2).filter(|w| w[0] == w[1]).to_set().len() >= 2
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let parsed = input.chars().map(|c| c as u8).to_vec();
    let mut pw = [0; 8];
    pw.copy_from_slice(&parsed);

    increment(&mut pw);
    while !is_valid(&pw) {
        increment(&mut pw);
    }
    increment(&mut pw);
    while !is_valid(&pw) {
        increment(&mut pw);
    }
    for v in pw {
        print!("{}", v as char);
    }
    println!();
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let parsed = input.chars().map(|c| c as u8).to_vec();
    let mut pw = [0; 8];
    pw.copy_from_slice(&parsed);

    increment(&mut pw);
    while !is_valid(&pw) {
        increment(&mut pw);
    }
    for v in pw {
        print!("{}", v as char);
    }
    println!();
}
