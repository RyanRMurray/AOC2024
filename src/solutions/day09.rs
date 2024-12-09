use std::collections::HashMap;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;
pub struct Day09Solution {}

pub fn day09(input: &str) -> Result<f32> {
    solve_linear::<Day09Solution, _, _, _>(input)
}

#[derive(Default, Debug, Clone)]
struct Files {
    allocated: HashMap<usize, Vec<usize>>,
    unallocated: Vec<usize>,
}

/// split unallocated space into contiguous chunks
fn to_contigs(ixs: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut contigs = vec![];
    let mut next_contig: Vec<usize> = vec![];

    for i in ixs {
        if next_contig.is_empty() || i.checked_sub(next_contig[next_contig.len() - 1]) == Some(1) {
            next_contig.push(*i);
        } else {
            contigs.push(next_contig);
            next_contig = vec![*i];
        }
    }

    contigs
}

/// add a newly freed chunk of contiguous space to the unallocated list
fn insert_contig(contigs: &mut Vec<Vec<usize>>, new: Vec<usize>) {
    if new.is_empty() {
        return;
    }
    for i in 0..contigs.len() {
        if !contigs[i].is_empty() && contigs[i][0] > new[0] {
            contigs.insert(i, new);
            return;
        }
    }
}

/// find enough contiguous free space to insert a file of size `size`
fn find_contig(contigs: &[Vec<usize>], size: usize) -> Option<usize> {
    contigs
        .iter()
        .enumerate()
        .find(|(_, c)| !c.is_empty() && c.len() >= size)
        .map(|(i, _)| i)
}

/// part 2 - compress by moving files to left-most available contiguous space
fn block_compress(mut files: Files) -> Files {
    let mut f = files.allocated.len() - 1;
    files.unallocated.reverse();
    let mut contigs = to_contigs(&files.unallocated);

    while f > 0 {
        let ixs = files.allocated.get(&f).unwrap().clone();

        match find_contig(&contigs, ixs.len()) {
            None => (),
            Some(i) => {
                if contigs[i][0] < ixs[0] {
                    let x = contigs[i].clone();

                    let (used, free) = x.split_at(ixs.len());

                    contigs[i] = free.to_vec();
                    insert_contig(&mut contigs, files.allocated.get(&f).unwrap().to_vec());
                    files.allocated.insert(f, used.to_vec());
                }
            }
        }
        f -= 1;
    }

    files
}

fn compress(mut files: Files) -> Files {
    let mut f = files.allocated.len() - 1;

    while f > 0 {
        let mut ixs = files.allocated.get(&f).unwrap().clone();
        let mut moved_ixs = vec![];
        while let Some(ix) = ixs.pop() {
            if let Some(unalloc_ix) = files.unallocated.pop() {
                if ix < unalloc_ix {
                    files.unallocated.push(unalloc_ix);
                    moved_ixs.push(ix);
                } else {
                    moved_ixs.push(unalloc_ix);
                }
            } else {
                moved_ixs.push(ix);
            }
        }
        files.allocated.insert(f, moved_ixs);
        f -= 1;
    }

    files
}

fn checksum(files: &Files) -> usize {
    files
        .allocated
        .iter()
        .flat_map(|(k, vs)| vs.iter().map(move |v| k * v))
        .sum()
}

impl SolutionLinear<Files, usize, usize> for Day09Solution {
    fn load(input: &str) -> Result<Files> {
        Ok(input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .chunks(2)
            .into_iter()
            .zip(0_usize..)
            .flat_map(|(ch, idx)| {
                let digits = ch.collect_vec();
                if digits.len() == 1 {
                    vec![Some(idx); digits[0]]
                } else {
                    let mut o = vec![Some(idx); digits[0]];
                    o.append(&mut vec![None; digits[1]]);
                    o
                }
            })
            .enumerate()
            .fold(Files::default(), |mut files, (i, v)| {
                match v {
                    None => files.unallocated.insert(0, i),
                    Some(v) => files.allocated.entry(v).or_default().push(i),
                }
                files
            }))
    }

    fn part1(input: &mut Files) -> Result<usize> {
        let compressed = compress(input.clone());

        Ok(checksum(&compressed))
    }

    fn part2(input: &mut Files, _part_1_solution: usize) -> Result<usize> {
        let compressed = block_compress(input.clone());

        Ok(checksum(&compressed))
    }
}

#[cfg(test)]
mod tests {
    use super::Day09Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("2333133121414131402", 1928, 2858)]
    fn validate_day09(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day09Solution::load(input).unwrap();

        let p1 = Day09Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day09Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
