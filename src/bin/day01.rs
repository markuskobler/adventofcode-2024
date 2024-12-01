use std::collections::{BinaryHeap, HashMap};

use nom::{
    character::complete::{self, line_ending, space1},
    combinator::opt,
    multi::fold_many1,
    sequence::{separated_pair, terminated},
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, (u32, u32)> {
    terminated(
        separated_pair(complete::u32, space1, complete::u32),
        opt(line_ending),
    )(input)
}

fn parse_part1(input: &str) -> IResult<&str, u32> {
    let cap = input.len() / 14;

    let (remaining, (n1, n2)) = fold_many1(
        parse_line,
        move || {
            (
                BinaryHeap::with_capacity(cap),
                BinaryHeap::with_capacity(cap),
            )
        },
        |(mut v1, mut v2), (n1, n2)| {
            v1.push(n1);
            v2.push(n2);
            (v1, v2)
        },
    )(input)?;

    let total = n1
        .into_sorted_vec()
        .into_iter()
        .zip(n2.into_sorted_vec().into_iter())
        .fold(0, |total, (n1, n2)| total + n1.abs_diff(n2));

    Ok((remaining, total))
}

fn parse_part2(input: &str) -> IResult<&str, u32> {
    let cap = input.len() / 14;

    let (remaining, (left, right)) = fold_many1(
        parse_line,
        || (Vec::with_capacity(cap), HashMap::<u32, u32>::new()),
        |(mut v1, mut v2), (n1, n2)| {
            v1.push(n1);
            v2.entry(n2).and_modify(|c| *c += 1).or_insert(1);
            (v1, v2)
        },
    )(input)?;

    let total = left
        .into_iter()
        .fold(0, |total, n| (right.get(&n).unwrap_or(&0) * n) + total);

    Ok((remaining, total))
}

fn main() {
    let input = include_str!("../../data/day01.txt");

    let (_, part1) = parse_part1(input).unwrap();
    eprintln!("Part 1: {part1}");

    let (_, part2) = parse_part2(input).unwrap();

    eprintln!("Part 2: {part2}");
}
