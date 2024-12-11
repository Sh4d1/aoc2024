use rustc_hash::FxHashSet as HashSet;
use std::collections::BinaryHeap;

pub struct Input {
    grid: Vec<Vec<usize>>,
    heads: Vec<(usize, usize)>,
    h: usize,
    w: usize,
}
#[aoc_generator(day10)]
pub fn parse(input: &str) -> Input {
    let mut heads = Vec::new();
    let grid: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == '0' {
                        heads.push((i, j));
                    }
                    c.to_digit(10).unwrap() as usize
                })
                .collect()
        })
        .collect();

    Input {
        h: grid.len(),
        w: grid[0].len(),
        grid,
        heads,
    }
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[aoc(day10, part1)]
pub fn part1(input: &Input) -> usize {
    input.heads.iter().fold(0, |mut acc, (hi, hj)| {
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::default();
        heap.push((*hi, *hj));
        loop {
            if let Some((pi, pj)) = heap.pop() {
                if input.grid[pi][pj] == 9 {
                    if !visited.contains(&(pi, pj)) {
                        acc += 1;
                        visited.insert((pi, pj));
                    }
                    continue;
                }
                for d in DIRS {
                    let (npi, npj) = (pi.wrapping_add_signed(d.0), pj.wrapping_add_signed(d.1));
                    if npi >= input.h || npj >= input.w {
                        continue;
                    }
                    if input.grid[npi][npj] != input.grid[pi][pj] + 1 {
                        continue;
                    }
                    heap.push((npi, npj));
                }
            } else {
                break;
            }
        }
        acc
    })
}
#[aoc(day10, part2)]
pub fn part2(input: &Input) -> usize {
    input.heads.iter().fold(0, |mut acc, (hi, hj)| {
        let mut heap = BinaryHeap::new();
        heap.push((*hi, *hj));
        loop {
            if let Some((pi, pj)) = heap.pop() {
                if input.grid[pi][pj] == 9 {
                    acc += 1;
                    continue;
                }
                for d in DIRS {
                    let (npi, npj) = (pi.wrapping_add_signed(d.0), pj.wrapping_add_signed(d.1));
                    if npi >= input.h || npj >= input.w {
                        continue;
                    }
                    if input.grid[npi][npj] != input.grid[pi][pj] + 1 {
                        continue;
                    }
                    heap.push((npi, npj));
                }
            } else {
                break;
            }
        }
        acc
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 36)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 81)
    }
}
