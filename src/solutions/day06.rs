use std::collections::HashSet;

use crate::utils::{
    grid::Grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;

pub struct Day06Solution {}

pub fn day06(input: &str) -> Result<f32> {
    solve_linear::<Day06Solution, _, _, _>(input)
}

const DIRS: [Pt<2>; 4] = [Pt([0, -1]), Pt([1, 0]), Pt([0, 1]), Pt([-1, 0])];

struct Maze {
    grid: Grid<bool, 2>,
    guard: Pt<2>,
    destined_path: HashSet<Pt<2>>,
}

// return -> (path,is loop)
fn simulate(maze: &Maze, obs: Pt<2>) -> (HashSet<(Pt<2>, usize)>, bool) {
    let mut history: HashSet<(Pt<2>, usize)> = HashSet::new();
    let mut g = maze.guard;
    let mut dir = 0;

    while maze.grid.grid.contains_key(&g) {
        if history.contains(&(g, dir)) {
            return (history, true);
        }

        history.insert((g, dir));

        while maze.grid.get_def(&(g + DIRS[dir])) || obs == g + DIRS[dir] {
            dir = (dir + 1) % 4;
        }
        g += DIRS[dir];
    }

    (history, false)
}

impl SolutionLinear<Maze, usize, usize> for Day06Solution {
    fn load(input: &str) -> Result<Maze> {
        let mut guard = Pt::<2>::default();

        let mut pairs = vec![];
        for (y, line) in (0..).zip(input.split('\n')) {
            for (x, c) in (0..).zip(line.chars()) {
                pairs.push((vec![x, y], c == '#'));
                if c == '^' {
                    guard = Pt([x, y]);
                }
            }
        }

        Ok(Maze {
            grid: Grid::from(pairs),
            guard,
            destined_path: HashSet::new(),
        })
    }

    fn part1(maze: &mut Maze) -> Result<usize> {
        let destiny = simulate(maze, Pt([-1, -1]));
        // we record the first path so we know where to put obstructions in part 2
        maze.destined_path = destiny.0.iter().map(|(p, _)| *p).collect::<HashSet<_>>();

        Ok(maze.destined_path.len())
    }

    fn part2(maze: &mut Maze, _part_1_solution: usize) -> Result<usize> {
        Ok(maze
            .destined_path
            .iter()
            .filter(|obs| {
                if **obs == maze.guard {
                    false
                } else {
                    simulate(maze, **obs).1
                }
            })
            .count())
    }
}

#[cfg(test)]
mod tests {
    use super::Day06Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        41,
        6
    )]
    fn validate_day06(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day06Solution::load(input).unwrap();

        let p1 = Day06Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day06Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
