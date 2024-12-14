use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;
pub struct Day13Solution {}

pub fn day13(input: &str) -> Result<f32> {
    solve_linear::<Day13Solution, _, _, _>(input)
}

#[derive(Debug)]
struct Machine {
    a: [isize; 2],
    b: [isize; 2],
    prize: [isize; 2],
}

/// solving linear equations, we reorganise:
/// - Ax_1+Bx_2 = P_x
/// - Ay_1+By_2 = P_y
///
/// to get:
/// B = (P_y*x_1-P_x*y_1)/(y_2*x_2-x_2*y_1)
/// A = (P_x-B*x_2)/(x_1)
/// If B is not an integer, there is no solution!
fn solve_machine(m: &Machine, limit: bool) -> Option<usize> {
    let n = m.prize[1] * m.a[0] - m.prize[0] * m.a[1];
    let d = m.b[1] * m.a[0] - m.b[0] * m.a[1];
    if n % d != 0 {
        return None;
    }
    let b_presses = n.checked_div_euclid(d)?;

    let n2 = m.prize[0] - b_presses * m.b[0];
    if n2 % m.a[0] != 0 {
        return None;
    }
    let a_presses = (m.prize[0] - b_presses * m.b[0]).checked_div(m.a[0])?;

    if limit && (!(0..=100).contains(&b_presses) || !(0..=100).contains(&b_presses)) {
        None
    } else {
        //println!("{:?}: A: {}, B: {}", m, a_presses, b_presses);
        Some((3 * a_presses + b_presses).try_into().unwrap())
    }
}

impl SolutionLinear<Vec<Machine>, usize, usize> for Day13Solution {
    fn load(input: &str) -> Result<Vec<Machine>> {
        Ok(input
            .split("\n\n")
            .map(|m| {
                let ls = m.split('\n').take(3).collect_vec();
                let prize_line = ls[2].split('=').collect_vec();
                // I know engineers who use regex, and they're all cowards.
                Machine {
                    a: [ls[0][11..14].parse().unwrap(), ls[0][17..].parse().unwrap()],
                    b: [ls[1][11..14].parse().unwrap(), ls[1][17..].parse().unwrap()],
                    prize: [
                        prize_line[1][0..prize_line[1].len() - 3].parse().unwrap(),
                        prize_line[2].parse().unwrap(),
                    ],
                }
            })
            .collect())
    }

    fn part1(input: &mut Vec<Machine>) -> Result<usize> {
        Ok(input.iter().filter_map(|m| solve_machine(m, true)).sum())
    }

    fn part2(input: &mut Vec<Machine>, _part_1_solution: usize) -> Result<usize> {
        Ok(input
            .iter_mut()
            .map(|m| {
                m.prize = [
                    m.prize[0] + 10_000_000_000_000,
                    m.prize[1] + 10_000_000_000_000,
                ];
                m
            })
            .filter_map(|m| solve_machine(m, false))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day13Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        480,
        875318608908
    )]
    fn validate_day13(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day13Solution::load(input).unwrap();

        let p1 = Day13Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day13Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
