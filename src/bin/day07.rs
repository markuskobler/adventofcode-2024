use adventofcode_2024::prelude::*;
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{alt, opt, repeat, separated, separated_pair, terminated},
    error::ParserError,
    prelude::*,
};

fn main() {
    let input = include_str!("../../data/day07.txt");

    let part1 = process_part1::<()>.parse(input).expect("to parse");
    eprintln!("Part 1: ... {part1}");
}

fn process_part1<'i, E: ParserError<Stream<'i>>>(input: &mut Stream<'i>) -> PResult<u64, E> {
    let nums = parse_equations(input)?;

    Ok(nums
        .into_iter()
        .filter_map(|(total, nums)| {
            if is_calibrated(total, 0, &nums) {
                Some(total)
            } else {
                None
            }
        })
        .sum())
}

fn is_calibrated(expected: u64, sum: u64, nums: &[u64]) -> bool {
    if nums.len() == 0 {
        return expected == sum;
    }
    if sum > expected {
        return false;
    }
    let n = nums[0];
    if is_calibrated(expected, sum + n, &nums[1..]) {
        return true;
    }
    if sum > 0 && is_calibrated(expected, sum * n, &nums[1..]) {
        return true;
    }
    return false;
}

fn parse_equations<'i, E: ParserError<Stream<'i>>>(
    input: &mut Stream<'i>,
) -> PResult<Vec<(u64, Vec<u64>)>, E> {
    Ok(std::iter::from_fn(move || {
        terminated(
            separated_pair(
                dec_uint::<&str, u64, E>,
                ": ",
                separated(1.., dec_uint::<&str, u64, E>, ' '),
            ),
            opt(line_ending),
        )
        .parse_next(input)
        .ok()
    })
    .collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_example_input() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let equations = parse_equations::<()>.parse(input).unwrap();

        assert_eq!(equations[0], (190, vec![10, 19]));
        assert_eq!(equations[1], (3267, vec![81, 40, 27]));

        assert_eq!(is_calibrated(190, 0, &[10, 19]), true);
        assert_eq!(is_calibrated(3267, 0, &[81, 40, 27]), true);
        assert_eq!(is_calibrated(83, 0, &[17, 5]), false);
    }
}
