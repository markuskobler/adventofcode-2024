use adventofcode_2024::prelude::*;

use std::collections::HashSet;

use winnow::ascii::{dec_uint, line_ending, newline};
use winnow::combinator::{opt, separated, separated_pair, terminated};
use winnow::prelude::*;
use winnow::{error::ParserError, PResult};

fn main() {
    let input = include_str!("../../data/day05.txt");

    let part1 = process_part1::<()>.parse(input).unwrap();
    eprintln!("Part 1: {part1}");
}

fn process_rules<'i, E: ParserError<Stream<'i>>>(
    input: &mut Stream<'i>,
) -> PResult<HashSet<(u32, u32)>, E> {
    Ok(std::iter::from_fn(move || {
        terminated(
            separated_pair(dec_uint::<&str, u32, E>, '|', dec_uint::<&str, u32, E>),
            opt(line_ending),
        )
        .parse_next(input)
        .ok()
    })
    .fold(HashSet::new(), |mut rules, (a, b)| {
        rules.insert((b, a));
        rules
    }))
}

fn process_pages<'i, E: ParserError<Stream<'i>>>(
    input: &mut Stream<'i>,
) -> PResult<Vec<Vec<u32>>, E> {
    Ok(std::iter::from_fn(move || {
        terminated(
            separated(1.., dec_uint::<&str, u32, E>, ","),
            opt(line_ending),
        )
        .parse_next(input)
        .ok()
    })
    .collect())
}

fn process_part1<'i, E: ParserError<Stream<'i>>>(input: &mut Stream<'i>) -> PResult<u32, E> {
    let (rules, pages) = separated_pair(process_rules, newline, process_pages).parse_next(input)?;
    Ok(pages
        .into_iter()
        .filter_map(move |page| {
            for (idx, up) in page[..page.len() - 1].iter().enumerate() {
                for down in page[idx + 1..].iter() {
                    if !rules.contains(&(*down, *up)) {
                        return None;
                    }
                }
            }
            Some(page[page.len() / 2])
        })
        .sum())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_rules() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let total = process_part1::<()>.parse(&input).unwrap();

        assert_eq!(total, 143);
    }
}
