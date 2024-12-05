use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Input {
    rules: HashSet<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

impl Input {
    pub fn sort_fn(&self) -> impl FnMut(&&usize, &&usize) -> std::cmp::Ordering {
        move |&a, &b| {
            if self.rules.contains(&(*a, *b)) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
    }
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Input {
    let (one, two) = input.split_once("\n\n").unwrap();
    Input {
        rules: one
            .lines()
            .map(|l| l.split_once('|').unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect(),
        updates: two
            .lines()
            .map(|l| l.split(",").map(|s| s.parse::<usize>().unwrap()).collect())
            .collect(),
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> usize {
    input
        .updates
        .iter()
        .filter(|u| {
            u.iter()
                .tuple_windows()
                .all(|(&a, &b)| input.rules.contains(&(a, b)))
        })
        .fold(0, |acc, u| acc + *u.get(u.len() / 2).unwrap())
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> usize {
    input
        .updates
        .iter()
        .filter(|u| {
            !u.iter()
                .tuple_windows()
                .all(|(&a, &b)| input.rules.contains(&(a, b)))
        })
        .fold(0, |acc, u| {
            acc + u
                .iter()
                .sorted_by(input.sort_fn())
                .nth(u.len() / 2)
                .unwrap()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 143)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 123)
    }
}
