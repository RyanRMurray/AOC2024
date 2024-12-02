use crate::utils::{
    load_input::{load_lines, load_segmented_lines},
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day02Solution {}

pub fn day02(input: &str) -> Result<f32> {
    solve_linear::<Day02Solution, _, _, _>(input)
}

fn is_safe(line: &Vec<usize>) -> bool {
    let increasing = &line[0].gt(&line[1]);

    line.iter().tuple_windows().all(|(a, b)| {
        match *increasing == a.gt(b) {
            false => return false,
            true => (),
        };
        let diff = a.abs_diff(*b);
        diff > 0 && diff < 4
    })
}

fn cooler_is_safe(line: &Vec<usize>) -> bool {
    // lazy :)
    let mut muts = vec![line.clone()];
    for i in 0..line.len() {
        let mut cl = line.clone();
        cl.remove(i);
        muts.push(cl);
    }

    muts.iter().any(|l| is_safe(l))
}

impl SolutionLinear<Vec<Vec<usize>>, usize, usize> for Day02Solution {
    fn load(input: &str) -> Result<Vec<Vec<usize>>> {
        Ok(load_lines(input, |l| {
            l.split(' ').map(|n| n.parse().unwrap()).collect()
        }))
    }

    fn part1(input: &mut Vec<Vec<usize>>) -> Result<usize> {
        Ok(input.iter().filter(|l| is_safe(l)).count())
    }

    fn part2(input: &mut Vec<Vec<usize>>, _part_1_solution: usize) -> Result<usize> {
        Ok(input.iter().filter(|l| cooler_is_safe(l)).count())
    }
}

#[cfg(test)]
mod tests {
    use super::Day02Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        2,
        4
    )]
    fn validate_day02(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day02Solution::load(input).unwrap();

        let p1 = Day02Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day02Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
