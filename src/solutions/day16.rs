use std::{
    collections::{HashMap, HashSet},
    ops::Mul,
};

use crate::utils::{
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day16Solution {}

pub fn day16(input: &str) -> Result<f32> {
    solve_linear::<Day16Solution, _, _, _>(input)
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

    let pts = g.iter().filter(|p| {
        // find if step is a node
        let mut ns = [false; 4];
        for i in 0..4 {
            ns[i] = g.contains(&(**p + OFFS[i]));
        }
        if ns.iter().all(|v|*v){
            return true;
        }
        ns[0] != ns[2] || ns[1] != ns[3]
    }).collect_vec();

    for p in pts.iter() {
        let mut paths = [None, None, None, None];
        for i in 0..4 {
            let mut path = (1..)
                .map(|steps| **p + OFFS[i].mul(steps))
                .take_while(|st| g.contains(st) && !pts.contains(&st))
                .collect_vec();

            if !path.is_empty() {
                path.push(path.last().unwrap()+&OFFS[i]);
                paths[i] = Some(path);
            } else if g.contains(&&(**p+OFFS[i])){
                paths[i] = Some(vec![**p+OFFS[i]])
            }
        }
        edges.insert(**p, paths);
    }
    println!("{:?}", edges.get(&Pt([1,139])));
    edges
}

fn next_nodes(
    pt: &Pt<2>,
    bearing: usize,
    edges: &Edges,
    distances: &HashMap<Pt<2>, usize>,
    score: usize,
) -> Vec<(usize, usize, Vec<Pt<2>>)> {
    edges
        .get(pt)
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if n.is_none() {
                None
            } else if distances.contains_key(n.clone().unwrap().last().unwrap()) {
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
fn distances(start: &Pt<2>, edges: &Edges) -> HashMap<Pt<2>, usize> {
    // min dist of all nodes
    let mut distances = HashMap::new();
    distances.insert(*start, 0);

    let mut frontier = next_nodes(start, 0, edges, &distances, 0);

    while !frontier.is_empty() {
        frontier.sort_by(|(a,_, _), (b,_, _)| b.cmp(a));
        let (cost, bearing, next) = frontier.pop().unwrap();
        let steps = next.clone();

        if distances.contains_key(&steps.last().unwrap()) {
            continue;
        }

        distances.insert(*steps.last().unwrap(), cost);

        frontier.extend(next_nodes(steps.last().unwrap(), bearing, edges, &distances, cost));
    }

    return distances
}

impl SolutionLinear<State, usize, usize> for Day16Solution {
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

        // for x in edges{
        //     println!("{:?}\t{:?}", x.0, x.1);
        // }

        Ok(State { start, end, edges })
    }

    fn part1(input: &mut State) -> Result<usize> {
        let d = distances(&input.start, &input.edges);
        println!("{:?}", input.start);
        println!("{:?}: {:?}", Pt([139,1]), d.get(&Pt([139,17])));
        // for x in d{
        //     println!("{:?}:\t{}", x.0,x.1);
        // }
        Ok(*d.get(&input.end).unwrap())
    }

    fn part2(_input: &mut State, _part_1_solution: usize) -> Result<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day16Solution;
    use crate::utils::solver_types::SolutionLinear;
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
        2
    )]
    fn validate_day16(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day16Solution::load(input).unwrap();

        let p1 = Day16Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day16Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
