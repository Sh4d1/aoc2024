use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;
use std::cmp::max;

#[derive(Debug, Clone)]
pub struct Robot {
    p: (isize, isize),
    v: (isize, isize),
}

#[derive(Debug, Clone)]
pub struct Input {
    robots: Vec<Robot>,
    w: usize,
    h: usize,
}

#[aoc_generator(day14)]
pub fn parse(input: &str) -> Input {
    let mut w = 0;
    let mut h = 0;
    Input {
        robots: input
            .lines()
            .map(|l| {
                let (l, r) = l.split_once(" ").unwrap();
                let p = l.split_once("=").unwrap().1.split_once(",").unwrap();
                let p = (p.0.parse().unwrap(), p.1.parse().unwrap());
                let v = r.split_once("=").unwrap().1.split_once(",").unwrap();
                let v = (v.0.parse().unwrap(), v.1.parse().unwrap());
                w = max(w, p.0);
                h = max(h, p.1);

                Robot { p, v }
            })
            .collect(),
        w: w as usize + 1,
        h: h as usize + 1,
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> usize {
    let mut robots = input.robots.clone();
    for _ in 0..100 {
        robots.iter_mut().for_each(|r| {
            r.p.0 += r.v.0;
            r.p.1 += r.v.1;
            if r.p.0 >= input.w as isize || r.p.0 < 0 {
                r.p.0 = r.p.0.rem_euclid(input.w as isize);
            }
            if r.p.1 >= input.h as isize || r.p.1 < 0 {
                r.p.1 = r.p.1.rem_euclid(input.h as isize);
            }
        });
    }
    robots
        .iter()
        .fold(HashMap::default(), |mut acc, r| {
            if r.p.0 > input.w as isize / 2 {
                if r.p.1 > input.h as isize / 2 {
                    *acc.entry(0).or_insert(0) += 1;
                } else if r.p.1 < input.h as isize / 2 {
                    *acc.entry(1).or_insert(0) += 1;
                }
            } else if r.p.0 < input.w as isize / 2 {
                if r.p.1 > input.h as isize / 2 {
                    *acc.entry(2).or_insert(0) += 1;
                } else if r.p.1 < input.h as isize / 2 {
                    *acc.entry(3).or_insert(0) += 1;
                }
            };
            acc
        })
        .values()
        .fold(1, |acc, v| acc * v)
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> usize {
    let mut robots = input.robots.clone();
    for i in 1.. {
        robots.iter_mut().for_each(|r| {
            r.p.0 += r.v.0;
            r.p.1 += r.v.1;
            if r.p.0 >= input.w as isize || r.p.0 < 0 {
                r.p.0 = r.p.0.rem_euclid(input.w as isize);
            }
            if r.p.1 >= input.h as isize || r.p.1 < 0 {
                r.p.1 = r.p.1.rem_euclid(input.h as isize);
            }
        });
        if robots.iter().map(|r| r.p).all_unique() {
            return i;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "p=0,4 v=3,-3
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
p=9,5 v=-3,-3";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 12)
    }
}
