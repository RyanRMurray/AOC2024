use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use std::cmp::Ordering;

pub struct Day07Solution {}

pub fn day07(input: &str) -> Result<f32> {
    solve_linear::<Day07Solution, _, _, _>(input)
}

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn mul(a: usize, b: usize) -> usize {
    a * b
}

fn concat(a: usize, b: usize) -> usize {
    let places = 10_usize.pow(b.checked_ilog(10).unwrap_or(0) + 1);
    // println!("{} || {} = {}", a,b, (a*places) + b);
    (a * places) + b
}

fn validate(
    input: &(usize, Vec<usize>),
    allowed_ops: &[&dyn Fn(usize, usize) -> usize],
    sum: usize,
    idx: usize,
    op: &dyn Fn(usize, usize) -> usize,
) -> bool {
    let res = op(sum, input.1[idx]);
    let end_reached = idx == input.1.len() - 1;
    match (res.cmp(&input.0), end_reached) {
        // over-shot goal
        (Ordering::Greater, _) => false,
        // all numbers consumed, goal reached
        (Ordering::Equal, true) => true,
        // all numbers consumed, under goal
        (Ordering::Less, true) => false,
        // numbers remaining
        _ => allowed_ops
            .iter()
            .any(|next_op| validate(input, allowed_ops, res, idx + 1, next_op)),
    }
}

impl SolutionLinear<Vec<(usize, Vec<usize>)>, usize, usize> for Day07Solution {
    fn load(input: &str) -> Result<Vec<(usize, Vec<usize>)>> {
        Ok(input
            .lines()
            .map(|l| {
                let (a, b) = l.split_once(": ").unwrap();
                (
                    a.parse().unwrap(),
                    b.split(' ').map(|c| c.parse().unwrap()).collect(),
                )
            })
            .collect())
    }

    fn part1(input: &mut Vec<(usize, Vec<usize>)>) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|i| validate(i, &[&add, &mul], 0, 0, &add))
            .map(|(t, _)| t)
            .sum())
    }

    fn part2(input: &mut Vec<(usize, Vec<usize>)>, _part_1_solution: usize) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|i| validate(i, &[&concat, &add, &mul], 0, 0, &add))
            .map(|(t, _)| t)
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day07Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        3749,
        11387
    )]
    fn validate_day07(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day07Solution::load(input).unwrap();

        let p1 = Day07Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day07Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
