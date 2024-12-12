use std::collections::VecDeque;

use itertools::Itertools;
use rustc_hash::FxHashSet as HashSet;

#[derive(Debug, Clone)]
pub struct Input {
    grid: Vec<Vec<char>>,
    h: usize,
    w: usize,
    seen: HashSet<(usize, usize)>,
}

impl Input {
    pub fn get_around(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        [
            (p.0 + 1, p.1),
            (p.0.wrapping_sub_signed(1), p.1),
            (p.0, p.1 + 1),
            (p.0, p.1.wrapping_sub_signed(1)),
        ]
        .iter()
        .copied()
        .filter(|pk| pk.0 < self.h && pk.1 < self.w && self.grid[pk.0][pk.1] == self.grid[p.0][p.1])
        .collect()
    }

    pub fn find_island(&mut self, p: (usize, usize)) -> HashSet<(usize, usize)> {
        let mut q = VecDeque::new();
        let mut island = HashSet::default();
        q.push_back(p);
        island.insert(p);
        while let Some(p) = q.pop_front() {
            for pk in self.get_around(p) {
                if self.seen.insert(pk) {
                    island.insert(pk);
                    q.push_back(pk);
                }
            }
        }
        island
    }
}

#[aoc_generator(day12)]
pub fn parse(input: &str) -> Input {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    Input {
        h: grid.len(),
        w: grid[0].len(),
        grid,
        seen: HashSet::default(),
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &Input) -> usize {
    let mut input = input.clone();
    (0..input.h)
        .cartesian_product(0..input.w)
        .fold(0, |acc, p| {
            if input.seen.contains(&p) {
                return acc;
            }
            let island = input.find_island(p);
            acc + island.len()
                * island
                    .iter()
                    .map(|p| 4 - input.get_around(*p).len())
                    .sum::<usize>()
        })
}
#[aoc(day12, part2)]
pub fn part2(input: &Input) -> usize {
    let mut input = input.clone();
    (0..input.h)
        .cartesian_product(0..input.w)
        .fold(0, |acc, p| {
            if input.seen.contains(&p) {
                return acc;
            }
            let island = input.find_island(p);
            acc + island.len()
                * island
                    .iter()
                    .cartesian_product([(0, 1), (0, -1), (1, 0), (-1, 0)])
                    .fold(HashSet::default(), |mut acc, (p, dp)| {
                        if island.contains(&(
                            p.0.wrapping_add_signed(dp.0),
                            p.1.wrapping_add_signed(dp.1),
                        )) {
                            return acc;
                        }
                        let mut pk = *p;
                        loop {
                            let p_angle = (
                                pk.0.wrapping_add_signed(dp.0),
                                pk.1.wrapping_add_signed(dp.1),
                            );
                            let p_next = (
                                pk.0.wrapping_add_signed(dp.1),
                                pk.1.wrapping_add_signed(dp.0),
                            );
                            if island.contains(&p_angle) || !island.contains(&p_next) {
                                break;
                            }
                            pk.0 = pk.0.wrapping_add_signed(dp.1);
                            pk.1 = pk.1.wrapping_add_signed(dp.0);
                        }
                        acc.insert((pk, dp));
                        acc
                    })
                    .len()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 1930)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 1206)
    }
}
