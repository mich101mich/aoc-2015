use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    fn count_escaped(c: char) -> usize {
        match c {
            '\\' | '"' => 2,
            _ => 1,
        }
    }

    let parsed = input
        .lines()
        .map(|l| {
            let code_len = l.len();
            let extended_len = l.chars().map(count_escaped).sum::<usize>() + 2;
            extended_len - code_len
        })
        .sum::<usize>();

    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::verify,
        multi::many0_count,
        sequence::{delimited, preceded},
        IResult,
    };

    fn byte(input: &str) -> IResult<&str, &str> {
        alt((
            preceded(tag("\\"), tag("\\")),
            preceded(tag("\\"), tag("\"")),
            preceded(
                tag("\\x"),
                verify(take(2usize), |x: &str| {
                    x.chars().all(|c| c.is_ascii_hexdigit())
                }),
            ),
            verify(take(1usize), |x: &str| x != "\"" && x != "\\"),
        ))(input)
    }
    fn count_bytes(input: &str) -> IResult<&str, usize> {
        delimited(tag("\""), many0_count(byte), tag("\""))(input)
    }

    let parsed = input
        .lines()
        .map(|l| {
            let code_len = l.len();
            let byte_len = count_bytes(l).unwrap().1;
            code_len - byte_len
        })
        .sum::<usize>();

    pv!(parsed);
}
