use std::{collections::HashMap, ops::Div};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;

pub struct Day11Solution {}

pub fn day11(input: &str) -> Result<f32> {
    solve_linear::<Day11Solution, _, _, _>(input)
}

// idea: rocks can be handled independently.
// idea: the outcome of a single rock is deterministic -> we can cache if we're smart.

// type (current rock value, steps remaining) -> outcomes where index is steps ahead
type RockLog = HashMap<(usize, usize), usize>;

fn split(rock: usize) -> (usize, usize) {
    let digits = digits(rock);
    let a = rock.div(10_usize.pow(digits / 2));
    let b = rock - a * 10_usize.pow(digits / 2);
    (a, b)
}

fn digits(rock: usize) -> u32 {
    rock.checked_ilog10().unwrap_or(0) + 1
}

fn process_rock(log: &mut RockLog, rock: usize, steps_remaining: usize) -> usize {
    //println!("{}", rock);
    // shortcut if final step. this is a rock! wow.
    if steps_remaining == 0 {
        return 1;
    }
    // if we've seen this before, return notified value
    if let Some(v) = log.get(&(rock, steps_remaining)) {
        return *v;
    }

    // process rock
    let res = match rock {
        0 => process_rock(log, 1, steps_remaining - 1),
        v if digits(v) % 2 == 0 => {
            let (a, b) = split(v);
            process_rock(log, a, steps_remaining - 1) + process_rock(log, b, steps_remaining - 1)
        }
        v => process_rock(log, v * 2024, steps_remaining - 1),
    };

    log.insert((rock, steps_remaining), res);
    res
}

fn solve(input: Vec<usize>, steps: usize) -> usize {
    let mut log = RockLog::new();
    let mut sum = 0;

    for rock in input {
        sum += process_rock(&mut log, rock, steps);
    }
    sum
}

impl SolutionLinear<Vec<usize>, usize, usize> for Day11Solution {
    fn load(input: &str) -> Result<Vec<usize>> {
        Ok(input.split(' ').map(|rock| rock.parse().unwrap()).collect())
    }

    fn part1(input: &mut Vec<usize>) -> Result<usize> {
        Ok(solve(input.to_vec(), 25))
    }

    fn part2(input: &mut Vec<usize>, _part_1_solution: usize) -> Result<usize> {
        Ok(solve(input.to_vec(), 75))
    }
}

#[cfg(test)]
mod tests {
    use super::{split, Day11Solution};
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(1000, 10, 0)]
    #[case(2024, 20, 24)]
    fn validate_split(#[case] input: usize, #[case] expected_1: usize, #[case] expected_2: usize) {
        let (a, b) = split(input);

        assert_eq!(expected_1, a);
        assert_eq!(expected_2, b);
    }

    #[rstest]
    #[case("125 17", 55312, 65601038650482)] // note, the second value wasn't provided by the AOC site
    fn validate_day11(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day11Solution::load(input).unwrap();

        let p1 = Day11Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day11Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
