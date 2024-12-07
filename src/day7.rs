use std::{collections::HashSet, str::pattern::Pattern};

#[derive(Debug)]
pub struct Equation {
    test: usize,
    equation: Vec<usize>,
}

#[derive(Debug)]
pub struct Input {
    equations: Vec<Equation>,
}

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Input {
    Input {
        equations: input
            .lines()
            .map(|l| {
                let (test, equation) = l.split_once(": ").unwrap();
                Equation {
                    test: test.parse().unwrap(),
                    equation: equation
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect(),
                }
            })
            .collect(),
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &Input) -> usize {
    input.equations.iter().fold(0, |acc, e| {
        let mut hs = HashSet::new();
        hs.insert(e.test);
        e.equation.iter().rev().for_each(|&n| {
            let add = hs
                .iter()
                .filter_map(|d| if n <= *d { Some(d - n) } else { None })
                .collect::<HashSet<_>>();
            let mul = hs
                .iter()
                .filter_map(|d| {
                    if d.rem_euclid(n) == 0 {
                        Some(d / n)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>();

            hs = add.union(&mul).copied().collect();
        });
        acc + hs.contains(&0).then(|| e.test).unwrap_or(0)
    })
}

#[aoc(day7, part2)]
pub fn part2(input: &Input) -> usize {
    input.equations.iter().fold(0, |acc, e| {
        let mut hs = HashSet::new();
        hs.insert(e.test);
        e.equation.iter().rev().for_each(|&n| {
            let add = hs
                .iter()
                .filter_map(|d| if n <= *d { Some(d - n) } else { None })
                .collect::<HashSet<_>>();
            let mul = hs
                .iter()
                .filter_map(|d| {
                    if d.rem_euclid(n) == 0 {
                        Some(d / n)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>();
            let concat = hs
                .iter()
                .filter_map(|d| {
                    if n.to_string().is_suffix_of(d.to_string().as_str()) {
                        Some(
                            d.to_string()
                                .strip_suffix(n.to_string().as_str())
                                .unwrap()
                                .parse()
                                .unwrap_or(0),
                        )
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>();

            hs = add.union(&mul).copied().collect();
            hs = hs.union(&concat).copied().collect();
        });
        acc + hs.contains(&0).then(|| e.test).unwrap_or(0)
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 3749)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 11387)
    }
}
