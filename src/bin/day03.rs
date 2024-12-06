use adventofcode_2024::prelude::*;

use winnow::combinator::{alt, delimited, separated_pair};
use winnow::prelude::*;
use winnow::token::one_of;
use winnow::{ascii::dec_uint, error::ParserError, PResult};

fn main() {
    let input = include_str!("../../data/day03.txt");

    let part1 = process_part1::<()>.parse(input).unwrap();
    eprintln!("Part 1: {part1}");

    let part2 = process_part2::<()>.parse(input).unwrap();
    eprintln!("Part 2: {part2}");
}

fn process_part1<'i, E: ParserError<Stream<'i>>>(input: &mut Stream<'i>) -> PResult<u32, E> {
    Ok(std::iter::from_fn(move || {
        alt((
            delimited(
                "mul(",
                separated_pair(dec_uint::<&str, u32, E>, ',', dec_uint::<&str, u32, E>),
                ")",
            )
            .map(|(a, b)| Some((a, b))),
            one_of::<_, _, E>(|_| true).map(|_| None),
        ))
        .parse_next(input)
        .ok()
    })
    .fold(0, |total, v| v.map_or(total, |(a, b)| total + (a * b))))
}

enum State {
    Corrupted,
    Toggle(bool),
    Mul((u32, u32)),
}

fn process_part2<'i, E: ParserError<Stream<'i>>>(input: &mut Stream<'i>) -> PResult<u32, E> {
    Ok(std::iter::from_fn(move || {
        alt((
            delimited(
                "mul(",
                separated_pair(dec_uint::<&str, u32, E>, ',', dec_uint::<&str, u32, E>),
                ")",
            )
            .map(|(a, b)| State::Mul((a, b))),
            "do()".map(|_| State::Toggle(true)),
            "don't()".map(|_| State::Toggle(false)),
            one_of::<_, _, E>(|_| true).map(|_| State::Corrupted),
        ))
        .parse_next(input)
        .ok()
    })
    .fold((0, true), |(total, on), v| match v {
        State::Corrupted => (total, on),
        State::Mul((a, b)) => {
            if on {
                (total + (a * b), on)
            } else {
                (total, on)
            }
        }
        State::Toggle(o) => (total, o),
    }))
    .map(|(total, _)| total)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing_parser() {
        let mut input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let testing = process_part1::<()>.parse(&mut input).unwrap();

        dbg!(testing);
    }
}
