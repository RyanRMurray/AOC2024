use std::cmp::Ordering;

use crate::utils::{
    grid::Grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day14Solution {}

pub fn day14(input: &str) -> Result<f32> {
    solve_linear::<Day14Solution, _, _, _>(input)
}

type Bots = Vec<(Pt<2>, Pt<2>)>;

fn adjust(start: isize, stepped: isize, bound: isize) -> isize {
    (bound + start + (stepped % bound)) % bound
}

fn simulate(state: &Bots, b_x: isize, b_y: isize, steps: isize) -> Bots {
    state
        .iter()
        .map(|(b_p, b_v)| {
            let adj_p_x = adjust(b_p.0[0], steps * b_v.0[0], b_x);
            let adj_p_y = adjust(b_p.0[1], steps * b_v.0[1], b_y);

            (Pt([adj_p_x, adj_p_y]), *b_v)
        })
        .collect()
}

fn calc_quads(bots: &Bots, quad_x: isize, quad_y: isize) -> [usize; 4] {
    bots.iter().fold([0, 0, 0, 0], |mut qs, (Pt([x, y]), _)| {
        match (x.cmp(&quad_x), y.cmp(&quad_y)) {
            (Ordering::Less, Ordering::Less) => qs[0] += 1,
            (Ordering::Greater, Ordering::Less) => qs[1] += 1,
            (Ordering::Less, Ordering::Greater) => qs[2] += 1,
            (Ordering::Greater, Ordering::Greater) => qs[3] += 1,
            _ => (),
        }

        qs
    })
}

#[allow(dead_code)]
fn print(bots: &Bots) -> String {
    let mut g = Grid::default();

    for (Pt([x, y]), _) in bots {
        *g.grid.entry(Pt([*x, *y])).or_default() += 1;
    }

    g.print(|v| match v {
        0 => '.',
        v => char::from_digit(v, 10).unwrap(),
    })
}

impl SolutionLinear<Bots, usize, usize> for Day14Solution {
    fn load(input: &str) -> Result<Bots> {
        Ok(input
            .lines()
            .map(|l| {
                let split = l.split('=').collect_vec();
                let (p_x, p_y) = split[1].split_once(',').unwrap();
                let (v_x, v_y) = split[2].split_once(',').unwrap();
                let p = Pt([p_x.parse().unwrap(), p_y[0..p_y.len() - 2].parse().unwrap()]);
                let v = Pt([v_x.parse().unwrap(), v_y.parse().unwrap()]);
                (p, v)
            })
            .collect())
    }

    fn part1(input: &mut Bots) -> Result<usize> {
        let (b_x, b_y) = if input.len() < 50 {
            (11, 7)
        } else {
            (101, 103)
        };
        let moved = simulate(input, b_x, b_y, 100);

        let quad_x = b_x / 2;
        let quad_y = b_y / 2;

        let quads = moved.iter().fold([0, 0, 0, 0], |mut qs, (Pt([x, y]), _)| {
            match (x.cmp(&quad_x), y.cmp(&quad_y)) {
                (Ordering::Less, Ordering::Less) => qs[0] += 1,
                (Ordering::Greater, Ordering::Less) => qs[1] += 1,
                (Ordering::Less, Ordering::Greater) => qs[2] += 1,
                (Ordering::Greater, Ordering::Greater) => qs[3] += 1,
                _ => (),
            }

            qs
        });
        println!("{:?}", quads);
        Ok(calc_quads(&moved, b_x / 2, b_y / 2).iter().product())
    }

    fn part2(input: &mut Bots, _part_1_solution: usize) -> Result<usize> {
        for i in 0..10_000 {
            let moved_b = simulate(input, 101, 103, i);
            if moved_b.iter().map(|(p, _)| p).unique().count() == input.len() {
                //println!("{}", print(&moved_b));
                return Ok(i.try_into().unwrap());
            }
        }

        panic!("No tree found.")
    }
}

#[cfg(test)]
mod tests {
    use super::Day14Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        12
    )]
    fn validate_day14(#[case] input: &str, #[case] expected_1: usize) {
        let mut input = Day14Solution::load(input).unwrap();

        let p1 = Day14Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);
    }
}
