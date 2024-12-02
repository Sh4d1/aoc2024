use itertools::{FoldWhile, Itertools};

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.split(" ").map(|c| c.parse().unwrap()).collect())
        .collect()
}
#[aoc(day2, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .filter(|&l| {
            l.iter()
                .tuple_windows()
                .fold_while((true, None), |mut acc, c: (&usize, &usize)| {
                    if acc.1.is_none() {
                        acc.1 = Some(c.1 > c.0);
                    }
                    if c.0 == c.1 || acc.1 != Some(c.1 > c.0) || c.1.abs_diff(*c.0) > 3 {
                        acc.0 = false;
                        return FoldWhile::Done(acc);
                    }
                    FoldWhile::Continue(acc)
                })
                .into_inner()
                .0
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .filter(|&l| {
            for i in 0..=l.len() {
                let mut new_l = l.clone();
                if i > 0 {
                    new_l.remove(i - 1);
                }
                if new_l
                    .iter()
                    .tuple_windows()
                    .fold_while((true, None), |mut acc, c: (&usize, &usize)| {
                        if acc.1.is_none() {
                            acc.1 = Some(c.1 > c.0);
                        }
                        if c.0 == c.1 || acc.1 != Some(c.1 > c.0) || c.1.abs_diff(*c.0) > 3 {
                            acc.0 = false;
                            return FoldWhile::Done(acc);
                        }
                        FoldWhile::Continue(acc)
                    })
                    .into_inner()
                    .0
                {
                    return true;
                }
            }
            false
        })
        .count()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 2)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 4)
    }
}
