use std::collections::HashMap;

use crate::utils::solver_types::{solve_simultaneous, SolutionSimultaneous};
use anyhow::Result;

pub struct Day19Solution {}

pub fn day19(input: &str) -> Result<f32> {
    solve_simultaneous::<Day19Solution, _, _, _>(input)
}

struct Towels {
    available: Vec<Vec<char>>,
    patterns: Vec<Vec<char>>,
}

// fn to_code(c: char) -> u8 {
//     match c {
//         'w' => 0,
//         'u' => 1,
//         'b' => 2,
//         'r' => 3,
//         'g' => 4,
//         c => panic!("Unrecognised color '{}'", c),
//     }
// }

fn valid(
    available: &Vec<Vec<char>>,
    cache: &mut HashMap<Vec<char>, usize>,
    pattern: &Vec<char>,
) -> Option<usize> {
    if pattern.is_empty() {
        return Some(1);
    }
    let subs = if let Some(v) = cache.get(pattern) {
        *v
    } else {
        let s = available
            .iter()
            .filter(|a| {
                if a.len() > pattern.len() {
                    false
                } else {
                    let cmp = &pattern[0..a.len()];

                    cmp == *a
                }
            })
            .filter_map(|v| valid(available, cache, &pattern[v.len()..].to_vec()))
            .sum();

        cache.insert(pattern.to_vec(), s);
        s
    };

    if subs == 0 {
        None
    } else {
        Some(subs)
    }
}

impl SolutionSimultaneous<Towels, usize, usize> for Day19Solution {
    fn load(input: &str) -> Result<Towels> {
        let (a, p) = input.split_once("\n\n").unwrap();
        let available = a.split(", ").map(|t| t.chars().collect()).collect();
        let patterns = p.lines().map(|l| l.chars().collect()).collect();

        Ok(Towels {
            available,
            patterns,
        })
    }

    fn solve(input: Towels) -> Result<(usize, usize)> {
        let mut cache = HashMap::new();
        let perms: Vec<usize> = input
            .patterns
            .iter()
            .filter_map(|p| valid(&input.available, &mut cache, p))
            .collect();
        Ok((perms.len(), perms.iter().sum()))
    }
}

#[cfg(test)]
mod tests {
    use super::Day19Solution;
    use crate::utils::solver_types::SolutionSimultaneous;
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
