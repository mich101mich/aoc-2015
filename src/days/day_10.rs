use crate::utils::*;
use std::mem::replace;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut parsed = input.chars().map(parse_c).to_vec();

    const END: usize = std::usize::MAX;

    // Solution with Generators: not that great until rust has native generators
    // let mut generator: Box<dyn Iterator<Item = usize>> = Box::new(parsed.into_iter());

    // struct Generator {
    //     inner: Box<dyn Iterator<Item = usize>>,
    //     current: usize,
    //     next_ret: Option<usize>,
    // };
    // impl Generator {
    //     fn new(mut inner: Box<dyn Iterator<Item = usize>>) -> Self {
    //         let current = inner.next().unwrap();
    //         Self {
    //             inner,
    //             current,
    //             next_ret: None,
    //         }
    //     }
    // }
    // impl Iterator for Generator {
    //     type Item = usize;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         if let Some(next_ret) = self.next_ret.take() {
    //             return Some(next_ret);
    //         } else if self.current == END {
    //             return None;
    //         }
    //         let mut count = 1;
    //         for w in self.inner.by_ref() {
    //             if w == self.current {
    //                 count += 1;
    //             } else {
    //                 self.next_ret = Some(replace(&mut self.current, w));
    //                 return Some(count);
    //             }
    //         }
    //         self.next_ret = Some(replace(&mut self.current, END));
    //         Some(count)
    //     }
    // }

    // for _ in 0..50 {
    //     generator = Box::new(Generator::new(generator));
    // }
    // let count = generator.count();
    // pv!(count);

    // alternative solution: Inverse Generators (visitor pattern)
    let mut current = END;
    let mut total = 0;
    let mut acceptor: Box<dyn FnMut(usize)> = Box::new(move |x: usize| {
        if x != current && replace(&mut current, x) != END {
            total += 2;
            if x == END {
                pv!(total);
            }
        }
    });

    for _ in 0..49 {
        let (mut count, mut current) = (0, END);
        acceptor = Box::new(move |x| {
            if x == current {
                count += 1;
            } else if current == END {
                current = x;
                count = 1;
            } else {
                acceptor(replace(&mut count, 1));
                acceptor(replace(&mut current, x));
                if x == END {
                    acceptor(END);
                }
            }
        });
    }
    for n in parsed {
        acceptor(n);
    }
    acceptor(END);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut parsed = input.chars().map(parse_c).to_vec();

    for _ in 0..40 {
        let mut next = vec![];
        let mut iter = parsed.into_iter();

        let mut v = iter.next().unwrap();
        let mut count = 1;
        for w in iter.chain(std::iter::once(99)) {
            if w == v {
                count += 1;
            } else {
                next.push(count);
                next.push(v);
                v = w;
                count = 1;
            }
        }

        parsed = next;
    }
    pv!(parsed.len());
}
