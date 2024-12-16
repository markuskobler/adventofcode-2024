use adventofcode_2024::prelude::*;

use std::collections::{BinaryHeap, HashMap};

use winnow::ascii::{dec_uint, line_ending, space1};
use winnow::combinator::{opt, repeat, separated_pair, terminated};
use winnow::error::ParserError;
use winnow::prelude::*;

fn parse_line<'i, E>(input: &mut Stream<'i>) -> PResult<(u32, u32), E>
where
    E: ParserError<&'i str>,
{
    terminated(separated_pair(dec_uint, space1, dec_uint), opt(line_ending)).parse_next(input)
}

fn parse_part1<'i, E>(input: &mut Stream<'i>) -> PResult<u32, E>
where
    E: ParserError<&'i str>,
{
    let cap = input.len() / 14;

    let (left, right) = repeat(1.., parse_line)
        .fold(
            move || {
                (
                    BinaryHeap::with_capacity(cap),
                    BinaryHeap::with_capacity(cap),
                )
            },
            |(mut left, mut right), (l, r)| {
                left.push(l);
                right.push(r);
                (left, right)
            },
        )
        .parse_next(input)?;

    let total = left
        .into_sorted_vec()
        .into_iter()
        .zip(right.into_sorted_vec())
        .fold(0, |total, (n1, n2)| total + n1.abs_diff(n2));

    Ok(total)
}

fn parse_part2<'i, E>(input: &mut Stream<'i>) -> PResult<u32, E>
where
    E: ParserError<&'i str>,
{
    let cap = input.len() / 14;

    let (left, right) = repeat(1.., parse_line)
        .fold(
            || (Vec::with_capacity(cap), HashMap::<u32, u32>::new()),
            |(mut v1, mut v2), (n1, n2)| {
                v1.push(n1);
                v2.entry(n2).and_modify(|c| *c += 1).or_insert(1);
                (v1, v2)
            },
        )
        .parse_next(input)?;

    Ok(left
        .into_iter()
        .fold(0, |total, n| (right.get(&n).unwrap_or(&0) * n) + total))
}

fn main() {
    let input = include_str!("../../data/day01.txt");

    let part1 = parse_part1::<()>.parse(input).unwrap();
    eprintln!("Part 1: {part1}");

    let part2 = parse_part2::<()>.parse(input).unwrap();
    eprintln!("Part 2: {part2}");
}
