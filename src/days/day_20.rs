use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let parsed = parse_u(input) / 11;

    for i in 1usize.. {
        let sum = (1..=50)
            .filter(|&d| i % d == 0)
            .map(|d| i / d)
            .sum::<usize>();
        if sum >= parsed {
            pv!(i);
            break;
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let parsed = parse_u(input) / 10;

    let mut primes: Vec<usize> = vec![];

    fn sum_divisors(n: usize, factors: &[(usize, usize)]) -> usize {
        let (&(p, e), factors) = match factors.split_first() {
            Some(x) => x,
            None => return n,
        };
        let iter = (0..=e).map(|i| n * p.pow(i as u32));
        if factors.is_empty() {
            iter.sum()
        } else {
            iter.map(|n| sum_divisors(n, factors)).sum()
        }
    }

    let mut factors = vec![];
    for i in 2usize.. {
        let mut n = i;
        factors.clear();
        for &p in primes.iter() {
            if p * p > i {
                break;
            }
            if n % p == 0 {
                let mut e = 1;
                n /= p;
                while n % p == 0 {
                    n /= p;
                    e += 1;
                }
                factors.push((p, e));
            }
        }
        let sum = if factors.is_empty() {
            primes.push(i);
            1 + i
        } else {
            sum_divisors(1, &factors)
        };
        if sum >= parsed {
            pv!(i);
            break;
        }
    }
}
