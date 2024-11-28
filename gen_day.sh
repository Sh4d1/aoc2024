#!/bin/bash

day=$1

export day

if [[ ! -f "./src/day${day}.rs" ]]; then
    envsubst <<EOF > ./src/day${day}.rs
#[aoc_generator(day$day)]
pub fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}
#[aoc(day$day, part1)]
pub fn part1(input: &[usize]) -> usize {
    0
}
#[aoc(day$day, part2)]
pub fn part2(input: &[usize]) -> usize {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 0)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 0)
    }
}
EOF
fi

if ! grep -q day${day} src/lib.rs; then
    sed -i "/aoc_lib.*/i pub mod day${day};" ./src/lib.rs
fi
