use std::collections::HashMap;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;

pub struct Day01Solution {}

pub fn day01(input: &str) -> Result<f32> {
    solve_linear::<Day01Solution, _, _, _>(input)
}

impl SolutionLinear<(Vec<usize>, Vec<usize>), usize, usize> for Day01Solution {
    fn load(input: &str) -> Result<(Vec<usize>, Vec<usize>)> {
        Ok(input.lines().fold((vec![], vec![]), |(mut l, mut r), i| {
            let (lx, rx) = i.split_once("   ").unwrap();
            l.push(lx.parse().unwrap());
            r.push(rx.parse().unwrap());
            (l, r)
        }))
    }

    // sort lists, sum diffs
    fn part1((l, r): &mut (Vec<usize>, Vec<usize>)) -> Result<usize> {
        l.sort();
        r.sort();
        Ok(l.iter()
            .zip(r.iter())
            .map(|(lx, rx)| lx.max(rx) - lx.min(rx))
            .sum())
    }

    fn part2((l, r): &mut (Vec<usize>, Vec<usize>), _part_1_solution: usize) -> Result<usize> {
        let mut occurrences = HashMap::new();
        for x in r{
            occurrences.entry(x).and_modify(|v| {*v += 1}).or_insert(1);
        }

        Ok(
            l.iter().map(|k| { k * occurrences.get(k).unwrap_or(&0)}).sum()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Day01Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "3   4
4   3
2   5
1   3
3   9
3   3",
        11,
        31
    )]
    fn validate(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day01Solution::load(input).unwrap();

        let p1 = Day01Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day01Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
