use std::ops::Mul;

use crate::utils::{
    grid::Grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day15Solution {}

pub fn day15(input: &str) -> Result<f32> {
    solve_linear::<Day15Solution, _, _, _>(input)
}

#[derive(Clone, Copy, Default, PartialEq)]
enum Obj {
    #[default]
    Wall,
    Box,
    BoxL,
    BoxR,
    Empty,
}

#[derive(Clone)]
struct State {
    g: Grid<Obj, 2>,
    bot: Pt<2>,
    instrs: Vec<Pt<2>>,
}

fn to_dir(c: char) -> Pt<2> {
    match c {
        '^' => Pt([0, -1]),
        '>' => Pt([1, 0]),
        'v' => Pt([0, 1]),
        '<' => Pt([-1, 0]),
        _ => panic!("unrecognised instr char"),
    }
}

fn calc_gps(Pt([x, y]): Pt<2>) -> usize {
    (x + y * 100).try_into().unwrap()
}

#[allow(dead_code)]
fn move_possible(maze: &Grid<Obj, 2>, bot_pos: &Pt<2>, dir: &Pt<2>) -> Option<Vec<Pt<2>>> {
    let mut ix = bot_pos + dir;
    let mut moved = vec![];
    while maze.get_def(&ix) == Obj::Box {
        moved.push(ix);
        ix += *dir;
    }
    if maze.get_def(&ix) == Obj::Empty {
        Some(moved)
    } else {
        None
    }
}

/// recursively find what is moved and whether it can be moved. Return `None` if any sub-object cannot be moved (that is, they'd collide with a wall)
fn wide_move_possible(maze: &Grid<Obj, 2>, bot_pos: &Pt<2>, dir: &Pt<2>) -> Option<Vec<Pt<2>>> {
    let horizontal = dir.0[0] != 0;
    match (horizontal, maze.get_def(&(bot_pos + dir))) {
        (_, Obj::Wall) => None,
        (_, Obj::Empty) => Some(vec![]),
        (_, Obj::Box) => {
            if let Some(mut ns) = wide_move_possible(maze, &(bot_pos + dir), dir) {
                ns.insert(0, bot_pos + dir);
                Some(ns)
            } else {
                None
            }
        }
        (true, _) => {
            // if pushing a wide box left or right
            if let Some(mut ns) = wide_move_possible(maze, &(bot_pos + &dir.mul(2)), dir) {
                ns.insert(0, bot_pos + dir);
                ns.insert(0, bot_pos + &dir.mul(2));
                Some(ns)
            } else {
                None
            }
        }
        (_, b) => {
            // if pushing the left/right side of a wide box up or down
            let offset = if b == Obj::BoxL {
                Pt([1, 0])
            } else {
                Pt([-1, 0])
            };
            if let (Some(mut ns1), Some(mut ns2)) = (
                wide_move_possible(maze, &(bot_pos + dir), dir),
                wide_move_possible(maze, &(bot_pos + dir + offset), dir),
            ) {
                ns1.append(&mut ns2);
                ns1.insert(0, bot_pos + dir);
                ns1.insert(0, bot_pos + dir + offset);
                Some(ns1)
            } else {
                None
            }
        }
    }
}

fn do_move(state: &mut State, dir: &Pt<2>, to_move: &[Pt<2>]) {
    state.bot += *dir;
    let mut moved = vec![];
    for ix in to_move.iter().unique() {
        moved.push(state.g.grid.shift_remove(ix).unwrap());
        state.g.grid.insert(*ix, Obj::Empty);
    }
    for (ix, m) in to_move.iter().unique().zip(moved) {
        state.g.grid.insert(ix + dir, m);
    }
}

fn simulate(state: &mut State, i: usize) {
    let d = state.instrs[i];

    if let Some(to_move) = wide_move_possible(&state.g, &state.bot, &d) {
        do_move(state, &d, &to_move);
    }
}

fn expand(state: &State) -> State {
    let (_, [max_x, max_y]) = state.g.bounds();

    let mut new_g = Grid::default();

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            let p = Pt([x, y]);
            let (o1, o2) = match state.g.get_def(&p) {
                Obj::Wall => (Obj::Wall, Obj::Wall),
                Obj::Box => (Obj::BoxL, Obj::BoxR),
                Obj::Empty => (Obj::Empty, Obj::Empty),
                _ => panic!("Cannot expand grid with [ or ]"),
            };
            new_g.grid.insert(Pt([x * 2, y]), o1);
            new_g.grid.insert(Pt([x * 2 + 1, y]), o2);
        }
    }

    State {
        g: new_g,
        bot: Pt([state.bot.0[0] * 2, state.bot.0[1]]),
        instrs: state.instrs.clone(),
    }
}

#[allow(dead_code)] //debugging function
fn print_grid(state: &State) {
    let (_, [max_x, max_y]) = state.g.bounds();

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            let p = Pt([x, y]);
            if p == state.bot {
                print!("@");
            } else {
                let c = match state.g.get_def(&p) {
                    Obj::Wall => '#',
                    Obj::Box => 'O',
                    Obj::BoxL => '[',
                    Obj::BoxR => ']',
                    Obj::Empty => '.',
                };
                print!("{}", c);
            }
        }
        println!();
    }
}

impl SolutionLinear<State, usize, usize> for Day15Solution {
    fn load(input: &str) -> Result<State> {
        let (g, is) = input.split_once("\n\n").unwrap();
        let mut bot_pos: Pt<2> = Pt([-1, -1]);
        let mut maze = Grid::default();

        for (uy, l) in g.lines().enumerate() {
            for (ux, c) in l.chars().enumerate() {
                let x = ux.try_into().unwrap();
                let y = uy.try_into().unwrap();
                let obj = match c {
                    '@' => {
                        bot_pos = Pt([x, y]);
                        Obj::Empty
                    }
                    '#' => Obj::Wall,
                    'O' => Obj::Box,
                    _ => Obj::Empty,
                };
                maze.grid.insert(Pt([x, y]), obj);
            }
        }

        let instrs = is
            .chars()
            .flat_map(|c| if c == '\n' { None } else { Some(to_dir(c)) })
            .collect_vec();

        Ok(State {
            g: maze,
            bot: bot_pos,
            instrs,
        })
    }

    fn part1(input: &mut State) -> Result<usize> {
        let mut state = input.clone();
        for i in 0..state.instrs.len() {
            simulate(&mut state, i);
            //println!("{}", i);
            //print_grid(input);
        }

        Ok(state
            .g
            .grid
            .iter()
            .flat_map(|(k, v)| match v {
                Obj::Box => Some(calc_gps(*k)),
                _ => None,
            })
            .sum())
    }

    fn part2(input: &mut State, _part_1_solution: usize) -> Result<usize> {
        let mut state = expand(input);
        for i in 0..state.instrs.len() {
            simulate(&mut state, i);
        }
        // print_grid(&state);

        Ok(state
            .g
            .grid
            .iter()
            .flat_map(|(k, v)| match v {
                Obj::BoxL => Some(calc_gps(*k)),
                _ => None,
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day15Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        10092,
        9021
    )]
    fn validate_day15(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day15Solution::load(input).unwrap();

        let p1 = Day15Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day15Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
