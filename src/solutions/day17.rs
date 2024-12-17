use std::ops::{BitXor, Div};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

pub struct Day17Solution {}

pub fn day17(input: &str) -> Result<f32> {
    solve_linear::<Day17Solution, _, _, _>(input)
}

#[derive(Debug, Clone)]
struct Machine {
    reg: [usize; 3],
    ptr: usize,
    program: Vec<usize>,
    out: Vec<usize>,
}

fn tonum(v: &Vec<usize>) -> usize {
    (0..v.len()).rev().zip(v).map(|(i, v)| v << (i * 3)).sum()
}

impl Machine {
    fn combo_val(&self, op: usize) -> usize {
        match op {
            0..=3 => op,
            4 => self.reg[0],
            5 => self.reg[1],
            6 => self.reg[2],
            _ => panic!("Invalid operand"),
        }
    }

    fn div(&mut self, reg: usize, op: usize) {
        self.reg[reg] = self.reg[0].div(2_usize.pow(op as u32));
    }

    fn output(&self) -> String {
        self.out.iter().map(|v| v.to_string()).join(",")
    }

    fn execute(&mut self, ptr: usize) -> Result<bool, ()> {
        let mut jump = true;
        if ptr >= self.program.len()
            || ([0, 2, 4, 5, 6, 7].contains(&self.program[ptr]) && ptr + 1 >= self.program.len())
        {
            Err(())
        } else {
            let operand = match self.program[ptr] {
                0 | 2 | 5 | 6 | 7 => self.combo_val(self.program[ptr + 1]),
                _ => self.program[ptr + 1],
            };

            match self.program[ptr] {
                0 => self.div(0, operand),
                6 => self.div(1, operand),
                7 => self.div(2, operand),
                1 => self.reg[1] = self.reg[1].bitxor(operand),
                2 => self.reg[1] = operand % 8,
                3 => {
                    if self.reg[0] == 0 {
                    } else {
                        jump = false;
                        self.ptr = operand
                    }
                }
                4 => self.reg[1] = self.reg[1].bitxor(self.reg[2]),
                5 => self.out.push(operand % 8),
                v => panic!("Unrecognised value {}", v),
            }

            Ok(jump)
        }
    }

    fn run(&mut self) -> String {
        while let Ok(ptr) = self.execute(self.ptr) {
            //println!("{:?}: {:?}", input.ptr, input.out);
            if ptr {
                self.ptr += 2;
            }
        }
        self.output()
    }
}

/// Part 2! so. the second example, and the real input, prints values that correspond to combinations of consecutive 3-bits in register A's value.
/// You could figure this out by giving A higher and higher values - it'll go up for every 3 powers of 2.
/// But there are multiple valid outputs, because some bit triplets depend on other bit triplets.
/// hence the recursion and reduction below.
fn solve(
    input: &Machine,
    target: &Vec<usize>,
    mut nums: Vec<usize>,
    i: usize,
) -> Option<Vec<usize>> {
    let mut valid = vec![];
    for val in 0..8 {
        let mut machine = input.clone();
        nums[i] = val;
        let reg_val = tonum(&nums);

        machine.reg[0] = reg_val;
        machine.run();
        let mut res = machine.out.clone();
        res.reverse();

        if res.len() == target.len() && res[i] == target[i] {
            //println!("{}: {:018b} -> {:?}", i, reg_val, machine.out);
            valid.push(nums.clone());
        }
    }
    if i == target.len() - 1 {
        // for v in valid.iter() {
        //     println!("{:?}\t{}", v, tonum(&v));
        // }
        valid.iter().min_by(|a, b| tonum(a).cmp(&tonum(b))).cloned()
    } else {
        valid
            .iter()
            .filter_map(|v| solve(input, target, v.clone(), i + 1))
            .min_by(|a, b| tonum(a).cmp(&tonum(b)))
    }
}

impl SolutionLinear<Machine, String, usize> for Day17Solution {
    fn load(input: &str) -> Result<Machine> {
        let ls = input.lines().collect_vec();
        let mut regs = [0; 3];
        for i in 0..3 {
            regs[i] = ls[i].split_once(": ").unwrap().1.parse().unwrap();
        }
        let program = ls
            .last()
            .unwrap()
            .split_once(' ')
            .unwrap()
            .1
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_vec();

        Ok(Machine {
            reg: regs,
            ptr: 0,
            program,
            out: vec![],
        })
    }

    fn part1(input: &mut Machine) -> Result<String> {
        let mut machine = input.clone();
        Ok(machine.run())
    }

    fn part2(input: &mut Machine, _part_1_solution: String) -> Result<usize> {
        let mut target = input.program.clone();
        target.reverse();
        let mut x = vec![1; target.len()];
        x[0] = 1;

        let vs = solve(input, &target, x, 0).unwrap();
        Ok(tonum(&vs))
    }
}

#[cfg(test)]
mod tests {
    use super::Day17Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        "4,6,3,5,6,3,5,2,1,0",
        None
    )]
    #[case(
        "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        "5,7,3,0",
        Some(117440)
    )]
    fn validate_day17(
        #[case] input: &str,
        #[case] expected_1: String,
        #[case] expected_2: Option<usize>,
    ) {
        let mut input = Day17Solution::load(input).unwrap();

        let p1 = Day17Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        if expected_2.is_some() {
            let p2 = Day17Solution::part2(&mut input, p1).unwrap();
            assert_eq!(expected_2.unwrap(), p2);
        }
    }
}
