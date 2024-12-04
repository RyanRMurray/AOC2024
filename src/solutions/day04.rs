use crate::utils::{
    grid::Grid,
    load_input::load_2d_grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day04Solution {}

const CROSS: [Pt<2>; 4] = [Pt([-1, -1]), Pt([1, -1]), Pt([-1, 1]), Pt([1, 1])];

pub fn day04(input: &str) -> Result<f32> {
    solve_linear::<Day04Solution, _, _, _>(input)
}

type XMASGrid = Grid<char, 2>;

fn check(g: &XMASGrid, pos: &Pt<2>, dir: &Pt<2>) -> bool {
    (1..)
        .zip("MAS".chars())
        .all(|(idx, c)| g.get_def(&(*pos + (*dir * idx))) == c)
}

fn checkx(g: &XMASGrid, pos: &Pt<2>) -> bool {
    if g.get_def(pos) != 'A' {
        return false;
    }

    let xs = CROSS.iter().map(|x| g.get_def(&(x + pos))).collect_vec();

    let first = (xs[0].max(xs[3]), xs[0].min(xs[3]));
    let second = (xs[1].max(xs[2]), xs[1].min(xs[2]));

    first == ('S', 'M') && second == ('S', 'M')
}

impl SolutionLinear<XMASGrid, usize, usize> for Day04Solution {
    fn load(input: &str) -> Result<XMASGrid> {
        Ok(load_2d_grid(input, |x| x))
    }

    fn part1(input: &mut XMASGrid) -> Result<usize> {
        Ok(input
            .grid
            .iter()
            .filter(|(_, v)| v == &&'X')
            .cartesian_product(Pt::<2>::neighbour_offsets().iter())
            .filter(|((k, _), dir)| check(input, k, dir))
            .count())
    }

    fn part2(input: &mut XMASGrid, _part_1_solution: usize) -> Result<usize> {
        Ok(input.grid.keys().filter(|k| checkx(input, k)).count())
    }
}

#[cfg(test)]
mod tests {
    use super::Day04Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        18,
        9
    )]
    fn validate_day04(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day04Solution::load(input).unwrap();

        let p1 = Day04Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day04Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
