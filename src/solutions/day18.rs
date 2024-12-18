use std::collections::HashSet;

use crate::utils::{
    load_input::load_lines,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::{anyhow, Result};
pub struct Day18Solution {}

pub fn day18(input: &str) -> Result<f32> {
    solve_linear::<Day18Solution, _, _, _>(input)
}

type Bytes = Vec<Pt<2>>;

/// bfs to end goal
fn navigate(dim: isize, bits: &HashSet<Pt<2>>) -> Result<usize> {
    let target = Pt([dim, dim]);
    let mut frontier = vec![(Pt([0, 0]), 0_usize)];
    let mut visited = HashSet::new();

    while !frontier.is_empty() {
        frontier.sort_by(|(_, a), (_, b)| b.cmp(a));
        let (next_p, next_c) = frontier.pop().unwrap();

        if visited.contains(&next_p) {
            continue;
        }

        visited.insert(next_p);
        if next_p == target {
            return Ok(next_c);
        }

        Pt::<2>::card_offsets().into_iter().for_each(|off| {
            let Pt([x, y]) = next_p + off;
            if 0 > x || dim < x || 0 > y || dim < y {
                return;
            }
            if !bits.contains(&Pt([x, y])) {
                if let Some(idx) = frontier
                    .iter()
                    .position(|(p, c)| *p == Pt([x, y]) && *c > next_c + 1)
                {
                    frontier.remove(idx);
                }
                frontier.push((Pt([x, y]), next_c + 1));
            }
        });
    }

    Err(anyhow!("No path found"))
}

impl SolutionLinear<Bytes, usize, String> for Day18Solution {
    fn load(input: &str) -> Result<Bytes> {
        Ok(load_lines(input, |l| {
            let (x, y) = l.split_once(',').unwrap();
            Pt([x.parse().unwrap(), y.parse().unwrap()])
        }))
    }

    fn part1(input: &mut Bytes) -> Result<usize> {
        let (dim, bits) = if input.len() < 100 {
            (6, 12)
        } else {
            (70, 1024)
        };

        let bits_fallen = input.iter().cloned().take(bits).collect();

        navigate(dim, &bits_fallen)
    }

    /// solve part 2 with a binary search
    fn part2(input: &mut Bytes, _part_1_solution: usize) -> Result<String> {
        let (dim, bits) = if input.len() < 100 {
            (6, 12)
        } else {
            (70, 1024)
        };

        let mut l = bits;
        let mut r = input.len();
        let mut idx;

        loop {
            idx = (l + r) / 2;
            match navigate(dim, &input.iter().cloned().take(idx).collect()) {
                Ok(_) => {
                    if navigate(dim, &input.iter().cloned().take(idx + 1).collect()).is_err() {
                        break;
                    }
                    l = idx;
                }
                Err(_) => {
                    if navigate(dim, &input.iter().cloned().take(idx - 1).collect()).is_ok() {
                        idx -= 1;
                        break;
                    }
                    r = idx
                }
            }
        }

        let Pt([x, y]) = input[idx];
        Ok(format!("{},{}", x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::Day18Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
        22,
        "6,1"
    )]
    fn validate_day18(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: String) {
        let mut input = Day18Solution::load(input).unwrap();

        let p1 = Day18Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day18Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
