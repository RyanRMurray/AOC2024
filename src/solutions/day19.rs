use crate::utils::solver_types::{solve_simultaneous, SolutionSimultaneous};
use anyhow::Result;

pub struct Day19Solution {}

pub fn day19(input: &str) -> Result<f32> {
    solve_simultaneous::<Day19Solution, _, _, _>(input)
}

struct Towels {
    available: Vec<Vec<u8>>,
    patterns: Vec<Vec<u8>>,
}

fn to_code(c: char) -> u8 {
    match c {
        'w' => 0,
        'u' => 1,
        'b' => 2,
        'r' => 3,
        'g' => 4,
        c => panic!("Unrecognised color '{}'", c),
    }
}

fn valid(available: &Vec<Vec<u8>>, pattern: &Vec<u8>, idx: usize) -> Option<usize> {
    if idx == pattern.len() {
        return Some(1);
    }

    let subs = available
        .iter()
        .filter(|a| {
            if a.len() + idx > pattern.len() {
                false
            } else {
                let cmp = &pattern[idx..idx + a.len()];

                cmp == *a
            }
        })
        .filter_map(|v| valid(available, pattern, idx + v.len()))
        .sum();

    if subs == 0 {
        None
    } else {
        Some(subs)
    }
}

impl SolutionSimultaneous<Towels, usize, usize> for Day19Solution {
    fn load(input: &str) -> Result<Towels> {
        let (a, p) = input.split_once("\n\n").unwrap();
        let available = a
            .split(", ")
            .map(|t| t.chars().map(to_code).collect())
            .collect();
        let patterns = p
            .lines()
            .map(|l| l.chars().map(to_code).collect())
            .collect();

        Ok(Towels {
            available,
            patterns,
        })
    }

    fn solve(input: Towels) -> Result<(usize, usize)> {
        let perms: Vec<usize> = input
            .patterns
            .iter()
            .filter_map(|p| valid(&input.available, &p, 0))
            .collect();

        Ok((perms.len(), perms.iter().sum()))
    }
}

#[cfg(test)]
mod tests {
    use super::Day19Solution;
    use crate::utils::solver_types::{SolutionLinear, SolutionSimultaneous};
    use rstest::rstest;

    #[rstest]
    #[case(
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        6,
        16
    )]
    fn validate_day19(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let input = Day19Solution::load(input).unwrap();

        let (p1, p2) = Day19Solution::solve(input).unwrap();
        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
