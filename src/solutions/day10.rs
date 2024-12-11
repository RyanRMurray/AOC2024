use crate::utils::{
    grid::Grid,
    load_input::load_2d_grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day10Solution {}

pub fn day10(input: &str) -> Result<f32> {
    solve_linear::<Day10Solution, _, _, _>(input)
}

type Map = Grid<u8, 2>;

fn find(m: &Map, find_val: &u8) -> Vec<Pt<2>> {
    m.grid
        .iter()
        .filter(|(_, v)| *v == find_val)
        .map(|(k, _)| *k)
        .collect_vec()
}

fn resolve_trail(m: &Map, trail: Vec<&Pt<2>>) -> Vec<Pt<2>> {
    let at = *trail.last().unwrap();
    if let Some(9) = m.grid.get(at) {
        vec![*at]
    } else {
        Pt::<2>::card_offsets()
            .iter()
            .map(|v| v + at)
            .filter(|n| m.get_def(n).checked_sub(m.get_def(at)) == Some(1) && !trail.contains(&n))
            .flat_map(|n| {
                let mut next_trail = trail.clone();
                next_trail.push(&n);
                resolve_trail(m, next_trail)
            })
            .collect_vec()
    }
}

impl SolutionLinear<Map, usize, usize> for Day10Solution {
    fn load(input: &str) -> Result<Map> {
        Ok(load_2d_grid(input, |c| {
            c.to_digit(10).unwrap().try_into().unwrap()
        }))
    }

    fn part1(input: &mut Map) -> Result<usize> {
        Ok(find(input, &0)
            .iter()
            .map(|at| resolve_trail(input, vec![at]).iter().unique().count())
            .sum())
    }

    fn part2(input: &mut Map, _part_1_solution: usize) -> Result<usize> {
        Ok(find(input, &0)
            .iter()
            .map(|at| resolve_trail(input, vec![at]).len())
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day10Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        36,
        81
    )]
    fn validate_day10(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day10Solution::load(input).unwrap();

        let p1 = Day10Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day10Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
