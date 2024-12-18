use rustc_hash::FxHashSet;
use std::collections::VecDeque;

pub struct Input {
    bytes: Vec<(usize, usize)>,
    h: usize,
    w: usize,
}
#[aoc_generator(day18)]
pub fn parse(input: &str) -> Input {
    let mut h = 0;
    let mut w = 0;

    let bytes = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            w = std::cmp::max(w, x);
            h = std::cmp::max(h, y);
            (x, y)
        })
        .collect();
    Input {
        bytes,
        w: w + 1,
        h: h + 1,
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &Input) -> usize {
    let mut grid = vec![vec![false; input.w]; input.h];
    (0..1024).map(|i| input.bytes[i]).for_each(|(x, y)| {
        grid[x][y] = true;
    });
    shortest(input, &grid).unwrap()
}

#[aoc(day18, part2)]
pub fn part2(input: &Input) -> String {
    let mut grid = vec![vec![false; input.w]; input.h];
    for (x, y) in input.bytes.iter() {
        grid[*x][*y] = true;
        if shortest(input, &grid).is_none() {
            return format!("{x},{y}");
        }
    }
    unreachable!()
}

fn shortest(input: &Input, grid: &Vec<Vec<bool>>) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visisted = FxHashSet::default();
    queue.push_back((0, (0usize, 0usize)));
    visisted.insert((0, 0));

    while let Some((s, p)) = queue.pop_front() {
        if p == (input.w - 1, input.h - 1) {
            return Some(s);
        }

        for d in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let np = (p.0.wrapping_add_signed(d.0), p.1.wrapping_add_signed(d.1));
            if np.0 >= input.w || np.1 >= input.h || grid[np.0][np.1] {
                continue;
            }
            if !visisted.contains(&np) {
                visisted.insert(np);
                queue.push_back((s + 1, np));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 0)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 0)
    }
}
