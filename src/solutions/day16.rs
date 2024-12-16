use std::{collections::{HashMap, HashSet}, ops::Mul};

use crate::utils::{
    grid::Grid,
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
const OFFS: [Pt<2>; 4] = [Pt([1,0]),Pt([0,1]),Pt([-1,0]), Pt([0,-1])];

type Reindeer = (Pt<2>, Pt<2>);

type Edges = HashMap<Pt<2>, [Option<(Pt<2>, usize)>;4]>;




/// return neighbours, the heading we would enter at, and the cost of entering their space
fn ns<'a>(at: &'a Pt<2>, score: usize, dir: &'a Pt<2>, g: &'a Grid<bool, 2>) -> impl Iterator<Item = (Pt<2>,Pt<2>,usize)> + 'a {
    Pt::<2>::card_offsets().into_iter().filter_map(move |off| {
        if g.get_def(&(at + &off)) {
            Some((at + &off, off, if off == *dir {score + 1} else {score + 1001}))
        } else {
            None
        }
    })
}

fn find_lowest(rd: &Reindeer, goal: &Pt<2>, g: &Grid<bool,2>) -> usize{
    let mut frontier = ns(&rd.0,0,&Pt([1,0]),g).collect_vec();

    while !frontier.is_empty(){
        //println!("{:?}", frontier);
        frontier.sort_by(|(_,_, a),(_,_, b)| b.cmp(a));
        let (next_at, next_heading, next_score) = frontier.pop().unwrap();
        if next_at == *goal{
            return next_score;
        }
        let next_frontier = ns(&next_at,next_score,&next_heading,g);

        for n in next_frontier{
            if let Some(idx) = frontier.iter().position(|(at,_,_)|at==&n.0){
                if frontier[idx].2 > n.2{
                    frontier[idx] = n;
                }
            } else {
                frontier.push(n);
            }
        }
    }
    panic!("could not reach goal")
}

/// follow direction until end
fn follow(at: &Pt<2>, dir:&Pt<2>, g: &HashSet<Pt<2>>) -> (Pt<2>, usize){
    (0_usize..).map(|steps|(at+&dir.mul(steps.try_into().unwrap()), steps)).take_while(|(next,_)| g.contains(next)).last().unwrap()
}


/// follow all directions til end, ignoring directions with no steps
fn follow_all(at: & Pt<2>, g: & HashSet<Pt<2>>) -> [Option<(Pt<2>, usize)>;4]{
    let mut res = [None, None, None, None];
    for i in 0..4{
        let next = follow(at, &OFFS[i], g);
        if next.1 != 0{
            res[i] = Some(next);

        }
    }
    res
}
 
/// simplify grid to inflection points
fn to_points(start: &Pt<2>, g: &HashSet<Pt<2>>) -> Edges{
    let mut frontier = vec![start.clone()];
    let mut visited = HashSet::new();
    let mut edges: Edges = HashMap::new();

    while !frontier.is_empty(){
        let at = frontier.pop().unwrap();
        if visited.contains(&at){
            continue;
        }

        // find neighbours
        let nexts = follow_all(&at,g);

        // record
        edges.insert(at, nexts);

        for n in nexts{
            if let Some(sn) = n{
                frontier.push(sn.0);
            }
        }

        visited.insert(at);
    }
    
    edges
}

fn get_nexts(from: Vec<Pt<2>>, bearing: usize, score: usize, g: &Edges) -> Vec<(Vec<Pt<2>>, usize)>{
    let ns = g.get(from.last().unwrap()).unwrap();
    let mut nexts = vec![];

    for (i,n) in ns.iter().enumerate(){
        if let Some(n) = n{
            if !from.contains(&n.0){
                let mut next = from.clone();
                let next_score = if i == bearing {score + n.1} else {score + n.1 + 1000};
                next.push(n.0);
                nexts.push((next, next_score));
            }
        }
    }
    nexts
}

fn find_lowests(start: &Pt<2>, end: &Pt<2>, edges: &Edges) ->Vec<(Vec<Pt<2>>, usize)>{
    let mut frontier = get_nexts(vec![start.clone()], 0, 0, edges);

    while !frontier.is_empty(){
        
    }

    todo!()

}


impl SolutionLinear<(Reindeer, Pt<2>, Edges), usize, usize> for Day16Solution {
    fn load(input: &str) -> Result<(Reindeer, Pt<2>, Edges)> {
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

        let edges = to_points(&rd.0, &g);

        Ok((rd, goal, edges))
    }

    fn part1((rd, goal,g): &mut (Reindeer, Pt<2>, Edges)) -> Result<usize> {
        for x in g.iter(){
            println!("{:?}: {:?}", x.0,x.1);
        }
        todo!()
    }

    fn part2(
        _input: &mut (Reindeer, Pt<2>, Edges),
        _part_1_solution: usize,
    ) -> Result<usize> {
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
