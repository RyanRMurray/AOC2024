use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    ops::Div,
};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;

pub struct Day05Solution {}

// Hashmap is VAL -> (VALS THAT COME AFTER)
type Instrs = (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>);

pub fn day05(input: &str) -> Result<f32> {
    solve_linear::<Day05Solution, _, _, _>(input)
}

fn sort_instr(rules: &HashMap<usize, HashSet<usize>>, instr: &mut [usize]) -> bool {
    let mut sort_occurred = false;
    instr.sort_by(|a, b| {
        if let Some(befores) = rules.get(a) {
            if befores.contains(b) {
                sort_occurred = true;
                return Ordering::Less;
            }
        }
        Ordering::Equal
    });
    sort_occurred
}

impl SolutionLinear<Instrs, usize, usize> for Day05Solution {
    fn load(input: &str) -> Result<Instrs> {
        let (rules, seqs) = input.split_once("\n\n").unwrap();

        let rules_proc = rules
            .lines()
            .fold(HashMap::<usize, HashSet<usize>>::new(), |mut m, l| {
                let (a, b) = l.split_once('|').unwrap();

                m.entry(a.parse().unwrap())
                    .or_default()
                    .insert(b.parse().unwrap());
                m
            });

        let seqs_proc = seqs
            .lines()
            .map(|l| l.split(',').map(|v| v.parse().unwrap()).collect())
            .collect();

        Ok((rules_proc, seqs_proc))
    }

    fn part1((rules, instrs): &mut Instrs) -> Result<usize> {
        let mut out = 0;

        for is in instrs {
            match sort_instr(rules, is) {
                true => (),
                false => out += is[is.len().div(2)],
            }
        }
        Ok(out)
    }

    fn part2((_, instrs): &mut Instrs, part_1_solution: usize) -> Result<usize> {
        // part 1 already sorted the vectors, so we just need all the numbers minus part 1
        Ok(instrs.iter().map(|is| is[is.len().div(2)]).sum::<usize>() - part_1_solution)
    }
}

#[cfg(test)]
mod tests {
    use super::Day05Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "47|53
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
97,13,75,29,47",
        143,
        123
    )]
    fn validate_day05(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day05Solution::load(input).unwrap();

        let p1 = Day05Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day05Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
