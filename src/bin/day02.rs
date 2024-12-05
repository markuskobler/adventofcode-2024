#![feature(iter_map_windows)]

use winnow::prelude::*;
use winnow::{
    ascii::{dec_uint, line_ending, space1},
    combinator::{opt, separated, terminated},
    error::ParserError,
    PResult,
};

pub fn main() {
    let input = include_str!("../../data/day02.txt");

    let part1 = process_part1::<()>.parse(input).unwrap();
    eprintln!("Part 1: {part1}");

    let part2 = process_part2::<()>.parse(input).unwrap();
    eprintln!("Part 2: {part2}");
}

type Stream<'i> = &'i str;

fn parse_line<'i, E: ParserError<Stream<'i>>>(input: &mut Stream<'i>) -> PResult<Vec<u32>, E> {
    terminated(
        separated(1.., dec_uint::<Stream<'i>, u32, E>, space1),
        opt(line_ending),
    )
    .parse_next(input)
}

fn process_part1<'i, E: ParserError<Stream<'i>>>(input: &mut Stream<'i>) -> PResult<usize, E> {
    Ok(
        std::iter::from_fn(move || parse_line::<()>.parse_next(input).ok())
            .filter_map(|nums| {
                nums.into_iter()
                    .map_windows(|[a, b]| (*a as i32 - *b as i32))
                    .try_fold(0, |last, delta| {
                        if (last == 0 || same_gradient(last, delta)) && within_range(delta) {
                            Some(delta)
                        } else {
                            None
                        }
                    })
            })
            .count(),
    )
}

#[derive(Debug)]
enum LastDelta {
    Start,
    Valid(i32),
    Invalid((i32, Option<i32>)),
}

fn process_part2<'i, E: ParserError<Stream<'i>>>(input: &mut Stream<'i>) -> PResult<usize, E> {
    Ok(
        std::iter::from_fn(move || parse_line::<()>.parse_next(input).ok())
            .filter_map(|nums| {
                nums.into_iter()
                    .map_windows(|[a, b]| (*b as i32 - *a as i32))
                    .try_fold(
                        (LastDelta::Start, -1),
                        |(last, retries), delta| match last {
                            LastDelta::Start => {
                                if within_range(delta) {
                                    Some((LastDelta::Valid(delta), 1))
                                } else {
                                    Some((LastDelta::Invalid((delta, None)), 0))
                                }
                            }
                            LastDelta::Invalid((first, None)) => {
                                Some((LastDelta::Invalid((delta, Some(first))), retries))
                            }
                            LastDelta::Invalid((p1, Some(p2))) => {
                                if within_range(delta)
                                    && (same_gradient(p1, delta) && within_range(p1)
                                        || (same_gradient(p2 + p1, delta) && within_range(p2 + p1)))
                                {
                                    Some((LastDelta::Valid(delta), retries))
                                } else if same_gradient(p1 + delta, p2)
                                    && within_range(p1 + delta)
                                    && within_range(p2)
                                {
                                    Some((LastDelta::Valid(p1 + delta), retries))
                                } else {
                                    None
                                }
                            }
                            LastDelta::Valid(first) => {
                                if same_gradient(first, delta) && within_range(delta) {
                                    Some((LastDelta::Valid(delta), retries))
                                } else if retries > 0 {
                                    Some((LastDelta::Invalid((delta, Some(first))), 0))
                                } else {
                                    None
                                }
                            }
                        },
                    )
            })
            .count(),
    )
}

#[inline]
fn within_range(n: i32) -> bool {
    (1..=3).contains(&n.abs())
}

#[inline]
fn same_gradient(a: i32, b: i32) -> bool {
    a ^ b >= 0
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_line() {
        let input = "7 6 4 2 1";
        let result = parse_line::<()>.parse(input).unwrap();
        assert_eq!(result, vec![7, 6, 4, 2, 1]);
    }

    #[rstest]
    #[case("7 6 4 2 1", 1)]
    #[case("1 2 7 8 9", 0)]
    #[case("9 7 6 2 1", 0)]
    #[case("1 3 2 4 5", 0)]
    #[case("8 6 4 4 1", 0)]
    #[case("1 3 6 7 9", 1)]
    #[case("10 16 17 20 23", 0)]
    fn part1(#[case] input: &str, #[case] count: usize) {
        let result = process_part1::<()>.parse(input);
        assert_eq!(result, Ok(count), "'{input}'");
    }

    #[rstest]
    #[case("1 4 3 2 1", 1)]
    #[case("1 6 7 8 9", 1)]
    #[case("1 1 2 3 4 5", 1)]
    #[case("1 2 3 4 5 5", 1)]
    #[case("5 1 2 3 4 5", 1)]
    #[case("7 10 8 10 11", 1)]
    #[case("29 28 27 26 25 22 20", 1)]
    #[case("48 46 47 49 51 54 56", 1)]
    #[case("29 31 34 40 42 45 47 48", 0)]
    #[case("34 37 38 40 42 46 43", 1)]
    #[case("85 89 86 87 89", 1)]
    #[case("10 12 12 9 7 4 2", 0)]
    #[case("59 61 59 61 63", 0)]
    #[case("72 75 73 73 73", 0)]
    #[case("95 95 93 95 98", 0)]
    #[case("61 66 70 71 71", 0)]
    #[case("30 24 22 15 12", 0)]
    #[case("68 67 69 66 65", 1)]
    #[case("43 40 44 46 47 48 50", 1)]
    #[case("51 46 46 44 41", 0)]
    #[case("23 19 15 14 11 10", 0)]
    #[case("56 57 58 58 64", 0)]
    #[case("30 24 22 15 12", 0)]
    #[case("38 37 34 35 28", 0)]
    #[case("10 16 17 20 23", 1)]
    #[case("70 70 73 79 80", 0)]
    #[case("36 29 26 29 30", 0)]
    #[case("69 70 69 66 63 60 58", 1)]
    #[case("14 12 9 6 4 3 5", 1)]
    #[case("26 33 30 32 34 36 39", 0)]
    #[case("43 40 41 44 45 46 48 51", 1)]
    #[case("85 89 86 87 89", 1)]
    fn valid_part2(#[case] input: &str, #[case] count: usize) {
        let result = process_part2::<()>.parse(input);
        assert_eq!(result, Ok(count), "'{input}'");
    }
}
