use std::collections::HashMap;

use itertools::Itertools;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|e| (e.0.parse::<usize>().unwrap(), e.1.parse::<usize>().unwrap()))
        .unzip()
}
#[aoc(day1, part1)]
pub fn part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    input
        .0
        .iter()
        .sorted()
        .zip(input.1.iter().sorted())
        .fold(0, |acc, e| acc + e.0.abs_diff(*e.1))
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let hm = input.1.iter().fold(HashMap::new(), |mut acc, l| {
        acc.insert(l, 1 + acc.get(&l).unwrap_or(&0));
        acc
    });
    input
        .0
        .iter()
        .fold(0, |acc, l| acc + l * hm.get(l).unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 11)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 31)
    }
}
