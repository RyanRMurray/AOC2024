use std::{
    collections::{HashMap, HashSet},
    ops::Mul,
};

use crate::utils::{
    point::Pt,
    solver_types::{solve_simultaneous, SolutionSimultaneous},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day16Solution {}

pub fn day16(input: &str) -> Result<f32> {
    solve_simultaneous::<Day16Solution, _, _, _>(input)
}

/// E S W N
const OFFS: [Pt<2>; 4] = [Pt([1, 0]), Pt([0, 1]), Pt([-1, 0]), Pt([0, -1])];

/// the linear set of points between two nodes
type Path = Vec<Pt<2>>;
type Edges = HashMap<Pt<2>, [Option<Path>; 4]>;

struct State {
    start: Pt<2>,
    end: Pt<2>,
    edges: Edges,
}

/// take a grid of points and turn them into edges of pt -> path in each direction if any
fn to_edges(g: HashSet<Pt<2>>) -> Edges {
    let mut edges = HashMap::new();

    let pts = g
        .iter()
        .filter(|p| {
            // find if step is a node
            let mut ns = [false; 4];
            for i in 0..4 {
                ns[i] = g.contains(&(**p + OFFS[i]));
            }

            if ns.iter().all(|v| *v) {
                return true;
            }

            ns[0] != ns[2] || ns[1] != ns[3]
        })
        .collect_vec();

    for p in pts.iter() {
        let mut paths = [None, None, None, None];
        for i in 0..4 {
            let mut path = (1..)
                .map(|steps| **p + OFFS[i].mul(steps))
                .take_while(|st| g.contains(st) && !pts.contains(&st))
                .collect_vec();

            if !path.is_empty() {
                path.push(path.last().unwrap() + &OFFS[i]);
                paths[i] = Some(path);
            } else if g.contains(&(**p + OFFS[i])) {
                paths[i] = Some(vec![**p + OFFS[i]])
            }
        }
        edges.insert(**p, paths);
    }
    println!("{:?}", edges.get(&Pt([1, 139])));
    edges
}

fn next_nodes(
    pt: &Pt<2>,
    bearing: usize,
    edges: &Edges,
    distances: Option<&HashMap<(Pt<2>, usize), usize>>,
    score: usize,
) -> Vec<(usize, usize, Vec<Pt<2>>)> {
    edges
        .get(pt)
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if n.is_none()
                || (distances.is_some()
                    && distances
                        .unwrap()
                        .contains_key(&(*n.clone().unwrap().last().unwrap(), i)))
            {
                None
            } else {
                let steps = n.clone().unwrap().clone();
                let next_score = if bearing == i {
                    score + steps.len()
                } else {
                    1000 + score + steps.len()
                };
                Some((next_score, i, steps.clone()))
            }
        })
        .collect_vec()
}

/// get the min distance of all nodes
fn distances(start: &Pt<2>, edges: &Edges) -> HashMap<(Pt<2>, usize), usize> {
    // min dist of all nodes
    let mut distances = HashMap::new();
    distances.insert((*start, 0), 0);

    let mut frontier = next_nodes(start, 0, edges, Some(&distances), 0);

    while !frontier.is_empty() {
        frontier.sort_by(|(a, _, _), (b, _, _)| b.cmp(a));
        let (cost, bearing, next) = frontier.pop().unwrap();
        let steps = next.clone();

        if distances.contains_key(&(*steps.last().unwrap(), bearing)) {
            continue;
        }

        distances.insert((*steps.last().unwrap(), bearing), cost);

        frontier.extend(next_nodes(
            steps.last().unwrap(),
            bearing,
            edges,
            Some(&distances),
            cost,
        ));
    }

    distances
}

fn dfs(
    (score, bearing, mut next): (usize, usize, Vec<Pt<2>>),
    max_score: usize,
    target: &Pt<2>,
    edges: &Edges,
    cached: &mut HashMap<(Pt<2>, usize, usize), Vec<Pt<2>>>,
) -> Option<Vec<Pt<2>>> {
    let pt = *next.last().unwrap();
    if let Some(res) = cached.get(&(pt, bearing, score)) {
        println!("hit");
        return Some(res.to_vec());
    }

    if score > max_score {
        return None;
    }

    if &pt == target {
        return Some(next);
    }

    let mut sub = next_nodes(&pt, bearing, edges, None, score)
        .into_iter()
        .filter_map(|n| dfs(n, max_score, target, edges, cached))
        .flatten()
        .collect_vec();

    if sub.is_empty() {
        //cached.insert((pt,bearing), vec![]);
        None
    } else {
        sub.append(&mut next);
        cached.insert((pt, bearing, score), sub.clone());
        Some(sub)
    }
}

impl SolutionSimultaneous<State, usize, usize> for Day16Solution {
    fn load(input: &str) -> Result<State> {
        let mut start = Pt([-1, -1]);
        let mut end = Pt([-1, -1]);
        let mut pts = HashSet::new();

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.char_indices() {
                let p = Pt([x.try_into().unwrap(), y.try_into().unwrap()]);
                match c {
                    'S' => start = p,
                    'E' => end = p,
                    '#' => continue,
                    _ => (),
                }
                pts.insert(p);
            }
        }
        let edges = to_edges(pts);

        Ok(State { start, end, edges })
    }

    fn solve(input: State) -> Result<(usize, usize)> {
        let d = distances(&input.start, &input.edges);
        let mut p1 = usize::MAX;
        for i in 0..4 {
            if let Some(v) = d.get(&(input.end, i)) {
                p1 = p1.min(*v);
            }
        }

        let visited = dfs(
            (0, 0, vec![input.start]),
            p1,
            &input.end,
            &input.edges,
            &mut HashMap::new(),
        )
        .unwrap();
        Ok((p1, visited.into_iter().unique().count()))
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

        let (p1, p2) = Day16Solution::solve(input).unwrap();
        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
