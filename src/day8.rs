use itertools::Itertools;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[derive(Default, Debug, Clone)]
pub struct Input {
    w: usize,
    h: usize,
    frequencies: HashMap<char, HashSet<(usize, usize)>>,
}

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Input {
    let mut frequencies: HashMap<char, HashSet<(usize, usize)>> = HashMap::default();
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c != '.' {
                        frequencies.entry(c).or_default().insert((i, j));
                    }
                    c
                })
                .collect()
        })
        .collect();
    Input {
        frequencies,
        w: grid[0].len(),
        h: grid.len(),
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> usize {
    input
        .frequencies
        .iter()
        .fold(HashSet::default(), |mut acc, fr| {
            acc.extend(fr.1.iter().tuple_combinations().fold(
                HashSet::default(),
                |mut acc, (a, b)| {
                    let new_a = ((2 * b.0).wrapping_sub(a.0), (2 * b.1).wrapping_sub(a.1));
                    let new_b = ((2 * a.0).wrapping_sub(b.0), (2 * a.1).wrapping_sub(b.1));
                    if new_a.0 < input.h && new_a.1 < input.w {
                        acc.insert(new_a);
                    }
                    if new_b.0 < input.h && new_b.1 < input.w {
                        acc.insert(new_b);
                    }
                    acc
                },
            ));
            acc
        })
        .len()
}
#[aoc(day8, part2)]
pub fn part2(input: &Input) -> usize {
    input
        .frequencies
        .iter()
        .fold(HashSet::default(), |mut acc, fr| {
            acc.extend(fr.1.iter().tuple_combinations().fold(
                HashSet::default(),
                |mut acc, (a, b)| {
                    let d1 = (a.0 as isize - b.0 as isize, a.1 as isize - b.1 as isize);
                    let d2 = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);

                    let mut next_a = *b;
                    let mut next_b = *a;
                    loop {
                        if next_a.0 >= input.h || next_a.1 >= input.w {
                            break;
                        }
                        acc.insert(next_a);
                        next_a = (
                            next_a.0.wrapping_sub_signed(d1.0),
                            next_a.1.wrapping_sub_signed(d1.1),
                        );
                    }

                    loop {
                        if next_b.0 >= input.h || next_b.1 >= input.w {
                            break;
                        }
                        acc.insert(next_b);
                        next_b = (
                            next_b.0.wrapping_sub_signed(d2.0),
                            next_b.1.wrapping_sub_signed(d2.1),
                        );
                    }
                    acc
                },
            ));
            acc
        })
        .len()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 14)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 34)
    }
}
