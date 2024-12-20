use std::collections::{HashMap, HashSet};

use crate::utils::{
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day20Solution {}

pub fn day20(input: &str) -> Result<f32> {
    solve_linear::<Day20Solution, _, _, _>(input)
}

type Maze = HashSet<Pt<2>>;
type Distances = HashMap<Pt<2>, usize>;
type Input = (Distances, Pt<2>, Pt<2>);
type Cheat = (Pt<2>, Pt<2>, usize);
/// NESW
const DIRS: [Pt<2>; 4] = [Pt([0, -1]), Pt([1, 0]), Pt([0, 1]), Pt([-1, 0])];

fn nexts<'a>(m: &'a Maze, pt: &'a Pt<2>, cost: usize) -> impl Iterator<Item = (Pt<2>, usize)> + 'a {
    DIRS.iter().filter_map(move |off| {
        let n = pt + off;
        if m.contains(&n) {
            Some((n, cost + 1))
        } else {
            None
        }
    })
}

fn to_distances(m: &Maze, e: &Pt<2>) -> Distances {
    let mut visited = HashMap::new();
    let mut frontier = vec![(*e, 0)];

    while !frontier.is_empty() {
        frontier.sort_by(|(_, a), (_, b)| b.cmp(a));
        let (next_p, next_c) = frontier.pop().unwrap();

        if visited.contains_key(&next_p) {
            continue;
        }

        visited.insert(next_p, next_c);

        let nexts = nexts(m, &next_p, next_c);
        for (n, c2) in nexts.into_iter() {
            if let Some(idx) = frontier.iter().position(|(p, c1)| *p == n && *c1 > c2) {
                frontier.remove(idx);
            }
            frontier.push((n, c2));
        }
    }

    visited
}

fn cheatable_n(d: &Distances, at: &Pt<2>, dist: isize) -> Vec<Cheat> {
    (-dist..=dist)
        .cartesian_product(-dist..=dist)
        .filter_map(|(x, y)| {
            let neighbour = at + &Pt([x, y]);
            let distance = x.abs() + y.abs();

            if distance > dist || !d.contains_key(&neighbour) {
                None
            } else {
                Some((*at, neighbour, distance as usize))
            }
        })
        .collect_vec()
}

/// how many seconds do we save if we cheat from `start` to `end`
fn faster_by(
    d: &Distances,
    fastest_time: usize,
    (cheat_start, cheat_end, cheat_distance): &Cheat,
) -> Option<usize> {
    let cheated_time = d.get(cheat_end).unwrap();
    let uncheated_time = d.get(cheat_start).unwrap();

    if (cheated_time + cheat_distance) >= *uncheated_time {
        return None;
    }

    Some(fastest_time - (cheated_time + cheat_distance + (fastest_time - uncheated_time)))
}

impl SolutionLinear<Input, usize, usize> for Day20Solution {
    fn load(input: &str) -> Result<Input> {
        let mut m = HashSet::new();
        let mut s = Pt([-1, -1]);
        let mut e = Pt([-1, -1]);
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.char_indices() {
                let pt = Pt([x.try_into().unwrap(), y.try_into().unwrap()]);
                match c {
                    'S' => s = pt,
                    'E' => e = pt,
                    _ => (),
                }

                if c != '#' {
                    m.insert(pt);
                }
            }
        }
        let d = to_distances(&m, &e);

        Ok((d, s, e))
    }

    fn part1((distances, start, _end): &mut Input) -> Result<usize> {
        let is_test = distances.len() < 225;

        let fastest_time = *distances.get(start).unwrap();

        let cheats = distances
            .keys()
            .flat_map(|p| cheatable_n(distances, p, 2))
            .filter_map(|cheat| faster_by(distances, fastest_time, &cheat))
            .collect_vec();

        if is_test {
            Ok(cheats.len())
        } else {
            Ok(cheats.iter().filter(|v| **v >= 100).count())
        }
    }

    fn part2((distances, start, _end): &mut Input, _part_1_solution: usize) -> Result<usize> {
        let is_test = distances.len() < 225;

        let fastest_time = *distances.get(start).unwrap();

        let cheats = distances
            .keys()
            .flat_map(|p| cheatable_n(distances, p, 20))
            .filter_map(|cheat| faster_by(distances, fastest_time, &cheat))
            .collect_vec();

        if is_test {
            Ok(cheats.len())
        } else {
            Ok(cheats.iter().filter(|v| **v >= 100).count())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day20Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
        44,
        3081
    )]
    fn validate_day20(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day20Solution::load(input).unwrap();

        let p1 = Day20Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day20Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
