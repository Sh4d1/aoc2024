use std::collections::HashSet;

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
        e.equation.iter().for_each(|&n| {
            if hs.len() == 0 {
                hs.insert(n);
                return;
            }

            let add = hs.iter().map(|d| d + n).collect::<HashSet<_>>();
            let mul = hs.iter().map(|d| d * n).collect::<HashSet<_>>();

            hs = add.union(&mul).copied().collect();
        });
        if hs.contains(&e.test) {
            acc + e.test
        } else {
            acc
        }
    })
}
#[aoc(day7, part2)]
pub fn part2(input: &Input) -> usize {
    input.equations.iter().fold(0, |acc, e| {
        let mut hs = HashSet::new();
        e.equation.iter().for_each(|&n| {
            if hs.len() == 0 {
                hs.insert(n);
                return;
            }

            let add = hs.iter().map(|d| d + n).collect::<HashSet<_>>();
            let mul = hs.iter().map(|d| d * n).collect::<HashSet<_>>();
            let concat = hs
                .iter()
                .filter_map(|d| {
                    let n_digit = n.checked_ilog10().unwrap_or(0) + 1;
                    let next = d * 10usize.pow(n_digit) + n;
                    if next > e.test { None } else { Some(next) }
                })
                .collect::<HashSet<_>>();

            hs = add.union(&mul).copied().collect();
            hs = hs.union(&concat).copied().collect();
        });
        if hs.contains(&e.test) {
            acc + e.test
        } else {
            acc
        }
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
