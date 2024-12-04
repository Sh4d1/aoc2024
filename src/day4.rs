use itertools::Itertools;

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}
#[aoc(day4, part1)]
pub fn part1(input: &[Vec<char>]) -> usize {
    let mut res = (0..input[0].len())
        .cartesian_product(0..input.len())
        .filter(|(x, y)| input[*y][*x] == 'X')
        .fold(Vec::new(), |mut acc, (x, y)| {
            acc.extend(
                (-1..=1isize)
                    .cartesian_product(-1..=1isize)
                    .filter(|(dx, dy)| (dx, dy) != (&0, &0))
                    .map(|(dx, dy)| (x, y, dx, dy)),
            );
            acc
        });
    ['M', 'A', 'S'].iter().for_each(|c| {
        res.retain_mut(|(x, y, dx, dy)| {
            *x = x.wrapping_add_signed(*dx);
            *y = y.wrapping_add_signed(*dy);
            if (*x >= input[0].len()) || (*y >= input.len()) {
                return false;
            }
            input[*y][*x] == *c
        });
    });
    res.len()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Vec<char>]) -> usize {
    (1..input[0].len() - 1)
        .cartesian_product(1..input.len() - 1)
        .filter(|(x, y)| input[*y][*x] == 'A')
        .filter(|(x, y)| {
            let around = [
                input[y - 1][x - 1],
                input[y + 1][x - 1],
                input[y - 1][x + 1],
                input[y + 1][x + 1],
            ];

            [
                ['M', 'M', 'S', 'S'],
                ['M', 'S', 'M', 'S'],
                ['S', 'S', 'M', 'M'],
                ['S', 'M', 'S', 'M'],
            ]
            .contains(&around)
        })
        .count()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 18)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 9)
    }
}
