use std::collections::{BinaryHeap, HashSet};

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum Cell {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn rotate_clockwise(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
            Dir::East => Dir::South,
        }
    }

    fn rotate_counterclockwise(&self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
            Dir::East => Dir::North,
        }
    }

    fn get_delta(&self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::South => (1, 0),
            Dir::West => (0, -1),
            Dir::East => (0, 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    grid: Vec<Vec<Cell>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[aoc_generator(day16)]
pub fn parse(input: &str) -> Input {
    let mut start = (0, 0);
    let mut end = (0, 0);
    Input {
        grid: input
            .lines()
            .enumerate()
            .map(|(i, c)| {
                c.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '#' => Cell::Wall,
                        '.' => Cell::Empty,
                        'S' => {
                            start = (i, j);
                            Cell::Empty
                        }
                        'E' => {
                            end = (i, j);
                            Cell::Empty
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
        start,
        end,
    }
}
#[aoc(day16, part1)]
pub fn part1(input: &Input) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited = FxHashMap::default();
    heap.push((0, input.start, Dir::East));

    while let Some((s, p, d)) = heap.pop() {
        let dp = d.get_delta();
        let next = (p.0.wrapping_add_signed(dp.0), p.1.wrapping_add_signed(dp.1));
        if input.end == p {
            return -s as usize;
        }

        if let Some(v) = visited.get(&(p, d)) {
            if *v > s {
                continue;
            }
        }
        visited.insert((p, d), s);

        match input.grid[next.0][next.1] {
            Cell::Empty => heap.push((s - 1, next, d)),
            Cell::Wall => (),
        }

        if let Some(v) = visited.get(&(p, d.rotate_clockwise())) {
            if *v < s - 1000 {
                heap.push((s - 1000, p, d.rotate_clockwise()));
            }
        } else {
            heap.push((s - 1000, p, d.rotate_clockwise()));
        }

        if let Some(v) = visited.get(&(p, d.rotate_counterclockwise())) {
            if *v < s - 1000 {
                heap.push((s - 1000, p, d.rotate_counterclockwise()));
            }
        } else {
            heap.push((s - 1000, p, d.rotate_counterclockwise()));
        }
    }
    unreachable!()
}
#[aoc(day16, part2)]
pub fn part2(input: &Input) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited = FxHashMap::default();
    let mut res = FxHashSet::default();
    heap.push((0, input.start, Dir::East, vec![input.start]));

    let mut score = 0;

    while let Some((s, p, d, prev)) = heap.pop() {
        if input.end == p {
            if score == 0 {
                score = s;
            }
            if s == score {
                for p in prev {
                    res.insert(p);
                }
                continue;
            }
            break;
        }

        if let Some(v) = visited.get(&(p, d)) {
            if *v > s {
                continue;
            }
        }
        visited.insert((p, d), s);

        let dp = d.get_delta();
        let next = (p.0.wrapping_add_signed(dp.0), p.1.wrapping_add_signed(dp.1));
        match input.grid[next.0][next.1] {
            Cell::Empty => {
                let mut prev = prev.clone();
                prev.push(next);
                heap.push((s - 1, next, d, prev));
            }
            Cell::Wall => (),
        }

        if let Some(v) = visited.get(&(p, d.rotate_clockwise())) {
            if *v <= s - 1000 {
                heap.push((s - 1000, p, d.rotate_clockwise(), prev.clone()));
            }
        } else {
            heap.push((s - 1000, p, d.rotate_clockwise(), prev.clone()));
        }

        if let Some(v) = visited.get(&(p, d.rotate_counterclockwise())) {
            if *v <= s - 1000 {
                heap.push((s - 1000, p, d.rotate_counterclockwise(), prev.clone()));
            }
        } else {
            heap.push((s - 1000, p, d.rotate_counterclockwise(), prev.clone()));
        }
    }

    res.len()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 7036);
        assert_eq!(part1(&parse(INPUT2)), 11048);
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 45);
        assert_eq!(part2(&parse(INPUT2)), 64);
    }
}
