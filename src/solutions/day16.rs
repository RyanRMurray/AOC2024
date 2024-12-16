use std::collections::{HashMap, HashSet};

use crate::utils::{
    grid::Grid,
    point::Pt,
    solver_types::{solve_simultaneous, SolutionSimultaneous},
};
use anyhow::Result;
use itertools::Itertools;
pub struct Day16Solution {}

pub fn day16(input: &str) -> Result<f32> {
    solve_simultaneous::<Day16Solution, _, _, _>(input)
}

type Reindeer = (Pt<2>, Pt<2>);

/// return neighbours, the heading we would enter at, and the cost of entering their space
fn ns<'a>(
    path: &'a Vec<Pt<2>>,
    score: usize,
    dir: &'a Pt<2>,
    g: &'a HashSet<Pt<2>>,
) -> impl Iterator<Item = (Vec<Pt<2>>, Pt<2>, usize)> + 'a {
    Pt::<2>::card_offsets().into_iter().filter_map(move |off| {
        let at = path.last().unwrap();
        if path.contains(&(at+&off)){
            return None;
        }
        if g.contains(&(at + &off)) {
            let mut new_path = path.clone();
            new_path.push(at + &off);
            Some((
                new_path,
                off,
                if off == *dir { score + 1 } else { score + 1001 },
            ))
        } else {
            None
        }
    })
}

fn find_lowest(rd: &Reindeer, goal: &Pt<2>, g: &HashSet<Pt<2>>) -> (Vec<Vec<Pt<2>>>, usize) {
    let mut target = Some(100_000);
    let mut shortests = vec![];
    let mut frontier = ns(&vec![rd.0], 0, &Pt([1, 0]), g).collect_vec();

    while !frontier.is_empty() {
        frontier.sort_by(|(_, _, a), (_, _, b)| b.cmp(a));
        let (path, next_heading, next_score) = frontier.pop().unwrap();
        let next_at = path.last().unwrap();

        println!("{:?}\t{}\t{:?}", frontier.len(), next_score, target);
        if target.is_some() && next_score > target.unwrap() {
            continue;
        }

        if next_at == goal {
            target = Some(next_score);
            shortests.push(path);
            continue;
        }
        let next_frontier = ns(&path, next_score, &next_heading, g);

        for n in next_frontier {
            if let Some(idx) = frontier.iter().position(|(at, _, _)| at == &n.0) {
                if frontier[idx].2 > n.2 {
                    frontier[idx] = n;
                }
            } else {
                frontier.push(n);
            }
        }
    }
    (shortests, target.unwrap())
}

impl SolutionSimultaneous<(Reindeer, Pt<2>, HashSet<Pt<2>>), usize, usize> for Day16Solution {
    fn load(input: &str) -> Result<(Reindeer, Pt<2>, HashSet<Pt<2>>)> {
        let mut rd = (Pt([-1, -1]), Pt([1, 0]));
        let mut goal = Pt([-1, -1]);
        let mut g = HashSet::new();
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.char_indices() {
                if c != '#' {
                    let p = Pt([x.try_into().unwrap(), y.try_into().unwrap()]);
                    match c {
                        'S' => rd.0 = p,
                        'E' => goal = p,
                        '.' => (),
                        bad => panic!("Unrecognised character {}", bad),
                    }
                    g.insert(p);
                }
            }
        }

        Ok((rd, goal, g))
    }

    fn solve((rd, goal, g): (Reindeer, Pt<2>, HashSet<Pt<2>>)) -> Result<(usize,usize)> {
        let (paths, min) = find_lowest(&rd, &goal, &g);

        Ok(
            (min,
            paths.iter().flatten().unique().count())
        )
    }

}

#[cfg(test)]
mod tests {
    use super::Day16Solution;
    use crate::utils::solver_types::SolutionSimultaneous;
    use rstest::rstest;

    #[rstest]
    #[case(
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        7036,
        45
    )]
    fn validate_day16(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let input = Day16Solution::load(input).unwrap();

        let (p1,p2) = Day16Solution::solve(input).unwrap();
        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
