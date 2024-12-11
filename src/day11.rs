use rustc_hash::FxHashMap as HashMap;
use std::collections::BinaryHeap;

pub struct Input {
    numbers: Vec<usize>,
    cache: HashMap<usize, Blink>,
}

enum Blink {
    Change(usize),
    Split(usize, usize),
}

#[aoc_generator(day11)]
pub fn parse(input: &str) -> Input {
    let mut cache = HashMap::default();
    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    let mut to_check: BinaryHeap<usize> = numbers.iter().copied().collect();

    while let Some(mut n) = to_check.pop() {
        while cache.get(&n).is_none() {
            if n == 0 {
                cache.insert(n, Blink::Change(1));
                n = 1;
                continue;
            }
            let size = n.ilog10() + 1;
            if size % 2 == 0 {
                let k = 10usize.pow(size / 2);
                let left = n / k;
                let right = n % k;
                cache.insert(n, Blink::Split(left, right));
                n = left;
                to_check.push(right);
            } else {
                cache.insert(n, Blink::Change(n * 2024));
                n = n * 2024;
            }
        }
    }
    Input { numbers, cache }
}

fn solve(input: &Input, n: usize) -> usize {
    (0..n)
        .fold(
            input.numbers.iter().map(|n| (n, 1)).collect(),
            |acc: HashMap<&usize, usize>, _| {
                acc.iter().fold(HashMap::default(), |mut acc, (n, c)| {
                    match input.cache.get(&n).unwrap() {
                        Blink::Change(d) => *acc.entry(d).or_insert(0) += c,
                        Blink::Split(l, r) => {
                            *acc.entry(l).or_insert(0) += c;
                            *acc.entry(r).or_insert(0) += c;
                        }
                    }
                    acc
                })
            },
        )
        .values()
        .sum()
}

#[aoc(day11, part1)]
pub fn part1(input: &Input) -> usize {
    solve(input, 25)
}

#[aoc(day11, part2)]
pub fn part2(input: &Input) -> usize {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "125 17";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 55312)
    }
}
