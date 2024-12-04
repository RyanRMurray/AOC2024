use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MUL_RE: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    static ref DO_RE: Regex = Regex::new(r"do\(\)").unwrap();
    static ref DONT_RE: Regex = Regex::new(r"don't\(\)").unwrap();
}

pub struct Day03Solution {}

pub fn day03(input: &str) -> Result<f32> {
    solve_linear::<Day03Solution, _, _, _>(input)
}

fn find_matches<'a>(input: &'a str, re: &'a Regex) -> impl Iterator<Item = (&'a str, usize)> + 'a {
    re.captures_iter(input).map(|c| {
        let cap = c.get(0).unwrap();
        (cap.as_str(), cap.start())
    })
}

fn process_mul(input: &str) -> usize {
    let (a, b) = input[4..input.len() - 1].split_once(',').unwrap();
    a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
}

impl SolutionLinear<String, usize, usize> for Day03Solution {
    fn load(input: &str) -> Result<String> {
        Ok(input.to_string())
    }

    fn part1(input: &mut String) -> Result<usize> {
        Ok(find_matches(input, &MUL_RE)
            .map(|(mul, _)| process_mul(mul))
            .sum())
    }

    fn part2(input: &mut String, _part_1_solution: usize) -> Result<usize> {
        // find locations for every mul, do, and dont. (assume a pre-string do and a post-string dont)
        let muls = find_matches(input, &MUL_RE).collect_vec();
        let mut dos = (0..1)
            .chain(find_matches(input, &DO_RE).map(|(_, ix)| ix))
            .peekable();
        let mut donts = find_matches(input, &DONT_RE)
            .map(|(_, ix)| ix)
            .chain(input.len()..input.len() + 1)
            .peekable();

        // create ranges of 'do' indexes by iterating over every do and matching them with a following dont
        let mut ranges = vec![];
        let mut opt_a = dos.next();
        let mut b = donts.next().unwrap();

        while let Some(a) = opt_a {
            if a < b {
                ranges.push((a, b));
                opt_a = dos.next();
            } else {
                b = donts.next().unwrap();
            }
        }

        // process a mul if their index is a 'do' index
        Ok(muls
            .iter()
            .filter_map(
                |(mul, ix)| match ranges.iter().any(|(a, b)| ix < b && ix > a) {
                    false => None,
                    true => Some(process_mul(mul)),
                },
            )
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day03Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        161,
        48
    )]
    fn validate_day03(
        #[case] input1: &str,
        #[case] input2: &str,
        #[case] expected_1: usize,
        #[case] expected_2: usize,
    ) {
        let mut input1 = Day03Solution::load(input1).unwrap();
        let mut input2 = Day03Solution::load(input2).unwrap();

        let p1 = Day03Solution::part1(&mut input1).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day03Solution::part2(&mut input2, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
