use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    Start,
}

pub struct Input {
    grid: Vec<Vec<Cell>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[aoc_generator(day20)]
pub fn parse(input: &str) -> Input {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Wall,
                    'S' => {
                        start = (i, j);
                        Cell::Start
                    }
                    'E' => {
                        end = (i, j);
                        Cell::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    Input { grid, start, end }
}

#[aoc(day20, part1)]
pub fn part1(input: &Input) -> usize {
    let mut p = input.start;
    let mut visited = vec![vec![0; input.grid[0].len()]; input.grid.len()];
    for i in 0.. {
        visited[p.0][p.1] = i;
        if p == input.end {
            break;
        }
        for dp in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let np = (p.0.wrapping_add_signed(dp.0), p.1.wrapping_add_signed(dp.1));
            if input.grid[np.0][np.1] == Cell::Empty && visited[np.0][np.1] == 0 {
                p = np;
                break;
            }
        }
    }

    let mut res = FxHashMap::default();
    for i in 1..(input.grid.len() - 1) {
        for j in 1..(input.grid[0].len() - 1) {
            if input.grid[i][j] != Cell::Wall {
                continue;
            }
            let mut before = ((i, j), std::usize::MAX);
            for dp in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let np = (i.wrapping_add_signed(dp.0), j.wrapping_add_signed(dp.1));
                if np.0 >= input.grid.len() || np.1 >= input.grid[0].len() {
                    continue;
                }
                if visited[np.0][np.1] != 0 && visited[np.0][np.1] < before.1 {
                    before = ((np.0, np.1), visited[np.0][np.1]);
                }
            }
            let mut after = ((i, j), 0);
            for dp in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let np = (i.wrapping_add_signed(dp.0), j.wrapping_add_signed(dp.1));
                if np.0 >= input.grid.len() || np.1 >= input.grid[0].len() {
                    continue;
                }
                if visited[np.0][np.1] != 0 && visited[np.0][np.1] > after.1 {
                    after = ((np.0, np.1), visited[np.0][np.1]);
                }
            }
            if after.1 == std::usize::MAX || before.1 == std::usize::MAX || after.1 == before.1 {
                continue;
            }

            *res.entry(after.1 - before.1 - 2).or_insert(0) += 1;
        }
    }

    res.iter()
        .fold(0, |acc, (k, v)| if *k >= 100 { acc + *v } else { acc })
}

#[aoc(day20, part2)]
pub fn part2(input: &Input) -> usize {
    let mut p = input.start;
    let mut visited = FxHashMap::default();
    for i in 0usize.. {
        visited.insert(p, i);
        if p == input.end {
            break;
        }
        for dp in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let np = (p.0.wrapping_add_signed(dp.0), p.1.wrapping_add_signed(dp.1));
            if input.grid[np.0][np.1] == Cell::Empty && !visited.contains_key(&np) {
                p = np;
                break;
            }
        }
    }

    visited
        .iter()
        .tuple_combinations()
        .fold(0, |acc, (c1, c2)| {
            let d = c1.0.0.abs_diff(c2.0.0) + c1.0.1.abs_diff(c2.0.1);
            if d <= 20 && c1.1.abs_diff(*c2.1) >= d + 100 {
                acc + 1
            } else {
                acc
            }
        })
}
