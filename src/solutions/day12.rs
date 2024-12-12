use std::{
    collections::{BTreeSet, HashSet},
    convert::identity,
};

use crate::utils::{
    grid::Grid,
    load_input::load_2d_grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

/// this one is pretty messy. mostly stream of consciousness. if i find time i'll neaten it up later.
pub struct Day12Solution {}

pub fn day12(input: &str) -> Result<f32> {
    solve_linear::<Day12Solution, _, _, _>(input)
}

const OFFS: [Pt<2>; 4] = [Pt([0, -1]), Pt([1, 0]), Pt([0, 1]), Pt([-1, 0])];

fn to_plots(input: &Grid<char, 2>) -> Vec<HashSet<Pt<2>>> {
    let mut pts: BTreeSet<Pt<2>> = input.grid.keys().cloned().collect();
    let mut plots = vec![];

    while !pts.is_empty() {
        let mut frontier = vec![pts.pop_first().unwrap()];
        let v = input.grid.get(&frontier[0]).unwrap();
        let mut new_plot = HashSet::new();

        while let Some(p) = frontier.pop()  {
            new_plot.insert(p);
            let _ = OFFS
                .iter()
                .map(|o| o + &p)
                .filter(|n| !new_plot.contains(n))
                .filter(|n| input.get_def(n) == *v)
                .map(|n| {
                    pts.remove(&n);
                    frontier.push(n)
                })
                .collect_vec();
        }
        plots.push(new_plot);
    }

    plots
}

fn price(plot: &HashSet<Pt<2>>) -> usize {
    // the perimiter is, for each point, the number of sides without a neighbouring point in the plot
    let perimiter: usize = plot
        .iter()
        .map(|p| OFFS.iter().filter(|o| !plot.contains(&(p + o))).count())
        .sum();
    perimiter * plot.len()
}

// check if the diagonal of a point is outside the plot
fn is_inner_corner(plot: &HashSet<Pt<2>>, p: &Pt<2>, d1: usize, d2: usize) -> bool {
    !plot.contains(&(p + &OFFS[d1] + OFFS[d2]))
}

fn count_corners(plot: &HashSet<Pt<2>>) -> usize {
    if plot.len() == 1 {
        return 4;
    }

    plot.iter()
        .map(|p| {
            let ns = OFFS
                .iter()
                .enumerate()
                .flat_map(|(i, o)| {
                    if plot.contains(&(o + p)) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .sorted()
                .collect_vec();
            match ns.len() {
                1 => 2,
                2 => {
                    if ns[1] - ns[0] == 2 {
                        // is line
                        0
                    } else if is_inner_corner(plot, p, ns[0], ns[1]) {
                        // has an inner corner
                        2
                    } else {
                        // is a box corner
                        1
                    }
                }
                _ => (0..ns.len())
                    .filter(|i| {
                        (ns[*i] + 1) % 4 == ns[(i + 1) % ns.len()]
                            && is_inner_corner(plot, p, ns[*i], ns[(i + 1) % ns.len()])
                    })
                    .count(),
            }
            //println!("{:?} -> {} ({:?})", p, x, ns);
        })
        .sum()
}

fn bulk_price(plot: &HashSet<Pt<2>>) -> usize {
    let sides = count_corners(plot);
    //println!("{:?} - {} * {}", plot, plot.len(), sides);

    sides * plot.len()
}

impl SolutionLinear<Vec<HashSet<Pt<2>>>, usize, usize> for Day12Solution {
    fn load(input: &str) -> Result<Vec<HashSet<Pt<2>>>> {
        Ok(to_plots(&load_2d_grid(input, identity)))
    }

    fn part1(input: &mut Vec<HashSet<Pt<2>>>) -> Result<usize> {
        Ok(input.iter().map(price).sum())
    }

    fn part2(input: &mut Vec<HashSet<Pt<2>>>, _part_1_solution: usize) -> Result<usize> {
        Ok(input.iter().map(bulk_price).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day12Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "AAAA
BBCD
BBCC
EEEC",
        140,
        80
    )]
    #[case(
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        1930,
        1206
    )]
    fn validate_day12(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day12Solution::load(input).unwrap();

        let p1 = Day12Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day12Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
