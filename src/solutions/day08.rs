use std::{
    collections::{HashMap, HashSet},
    ops::Div,
};

use crate::utils::{
    grid::Grid,
    load_input::load_2d_grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day08Solution {}

pub fn day08(input: &str) -> Result<f32> {
    solve_linear::<Day08Solution, _, _, _>(input)
}

struct G {
    x: isize,
    y: isize,
    locs: HashMap<char, Vec<Pt<2>>>,
}

fn project(Pt([ax, ay]): &Pt<2>, Pt([bx, by]): &Pt<2>) -> Pt<2> {
    Pt([bx + bx - ax, by + by - ay])
}

fn antinodes_for<'a>(locs: &'a [Pt<2>]) -> impl Iterator<Item = Pt<2>> + 'a {
    locs.iter()
        .cartesian_product(locs.iter())
        .filter(|(a, b)| a != b)
        .flat_map(|(a, b)| vec![project(a, b), project(b, a)])
}

impl SolutionLinear<G, usize, usize> for Day08Solution {
    fn load(input: &str) -> Result<G> {
        let mut locs: HashMap<char, Vec<Pt<2>>> = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in (0..).zip(input.split('\n')) {
            for (x, c) in (0..).zip(line.chars()) {
                if c != '.' {
                    locs.entry(c).or_default().push(Pt([x, y]));
                }
                max_x = x;
            }
            max_y = y;
        }
        Ok(G {
            x: max_x,
            y: max_y,
            locs,
        })
    }

    fn part1(input: &mut G) -> Result<usize> {
        Ok(input
            .locs
            .values()
            .flat_map(|v| antinodes_for(v))
            .filter(|Pt([a, b])| a >= &0 && a <= &input.x && b >= &0 && b <= &input.y)
            .unique()
            .count())
    }

    fn part2(input: &mut G, _part_1_solution: usize) -> Result<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day08Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        14,
        34
    )]
    fn validate_day08(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day08Solution::load(input).unwrap();

        let p1 = Day08Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day08Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
