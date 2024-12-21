use rustc_hash::FxHashMap;
use std::iter::once;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Push,
}
use Direction::*;

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Up => 0,
            Down => 1,
            Left => 2,
            Right => 3,
            Push => 4,
        }
    }
}

pub struct Input {
    codes: Vec<Vec<u8>>,
}

#[aoc_generator(day21)]
pub fn parse(input: &str) -> Input {
    Input {
        codes: input
            .lines()
            .map(|l| {
                l.strip_suffix("A")
                    .unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
    }
}

fn solve_code(code: &Vec<u8>) -> Vec<Vec<Direction>> {
    let mut start = 10;
    let mut res = Vec::new();
    for c in code.iter().chain(once(&10)) {
        let mut paths = get_numerical_path(start, *c as usize);
        paths.iter_mut().for_each(|p| p.push(Push));
        if res.is_empty() {
            res = paths;
        } else {
            let mut new_res = Vec::new();
            let mut min = std::usize::MAX;
            for old_path in res.into_iter() {
                let old_len = old_path.len();
                for path in &paths {
                    let mut old_path = old_path.clone();
                    let new_len = old_len + path.len();
                    if new_len < min {
                        min = new_len;
                        old_path.extend(path);
                        new_res = vec![old_path];
                    } else if new_len == min {
                        old_path.extend(path);
                        new_res.push(old_path.to_vec());
                    }
                }
            }
            res = new_res;
        }
        start = *c as usize;
    }
    res
}

fn shortest_path_size(
    dirs: Vec<Direction>,
    robots_to_go: usize,
    visited: &mut FxHashMap<(Vec<Direction>, usize, Direction), usize>,
    starts_pos: &mut Vec<Direction>,
) -> usize {
    let key = (dirs.clone(), robots_to_go, starts_pos[robots_to_go]);
    if let Some(known) = visited.get(&key) {
        return *known;
    }

    let v = dirs.iter().fold(0, |acc, dir| {
        let short_paths = get_directional_path(starts_pos[robots_to_go].into(), (*dir).into());
        starts_pos[robots_to_go] = *dir;
        acc + short_paths
            .into_iter()
            .map(|p| {
                if robots_to_go == 1 {
                    p.len()
                } else {
                    shortest_path_size(p, robots_to_go - 1, visited, starts_pos)
                }
            })
            .min()
            .unwrap()
    });
    visited.insert(key, v);
    v
}

fn solve(code: &Vec<u8>, n: usize) -> usize {
    let codes = solve_code(code);
    codes
        .into_iter()
        .map(|p| shortest_path_size(p, n, &mut FxHashMap::default(), &mut vec![Push; n + 1]))
        .min()
        .unwrap()
        * code
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, c)| acc + 10usize.pow(i as u32) * *c as usize)
}

#[aoc(day21, part1)]
pub fn part1(input: &Input) -> usize {
    input.codes.iter().fold(0, |acc, c| acc + solve(c, 2))
}

#[aoc(day21, part2)]
pub fn part2(input: &Input) -> usize {
    input.codes.iter().fold(0, |acc, c| acc + solve(c, 25))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "029A
980A
179A
456A
379A";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 126384)
    }
}

// Why ? I don't know, why not?

fn get_directional_path(start: usize, end: usize) -> Vec<Vec<Direction>> {
    let mut res = match (start, end) {
        (0, 0) => vec![vec![]],
        (0, 1) => vec![vec![Down]],
        (0, 2) => vec![vec![Down, Left]],
        (0, 3) => vec![vec![Down, Right], vec![Right, Down]],
        (0, 4) => vec![vec![Right]],
        (1, 0) => vec![vec![Up]],
        (1, 1) => vec![vec![]],
        (1, 2) => vec![vec![Left]],
        (1, 3) => vec![vec![Right]],
        (1, 4) => vec![vec![Up, Right], vec![Right, Up]],
        (2, 0) => vec![vec![Right, Up]],
        (2, 1) => vec![vec![Right]],
        (2, 2) => vec![vec![]],
        (2, 3) => vec![vec![Right, Right]],
        (2, 4) => vec![vec![Right, Up, Right], vec![Right, Right, Up]],
        (3, 0) => vec![vec![Up, Left], vec![Left, Up]],
        (3, 1) => vec![vec![Left]],
        (3, 2) => vec![vec![Left, Left]],
        (3, 3) => vec![vec![]],
        (3, 4) => vec![vec![Up]],
        (4, 0) => vec![vec![Left]],
        (4, 1) => vec![vec![Down, Left], vec![Left, Down]],
        (4, 2) => vec![vec![Down, Left, Left], vec![Left, Down, Left]],
        (4, 3) => vec![vec![Down]],
        (4, 4) => vec![vec![]],
        _ => unreachable!(),
    };
    res.iter_mut().for_each(|v| v.push(Push));
    res
}

fn get_numerical_path(start: usize, end: usize) -> Vec<Vec<Direction>> {
    match (start, end) {
        (0, 0) => vec![vec![]],
        (0, 1) => vec![vec![Up, Left]],
        (0, 2) => vec![vec![Up]],
        (0, 3) => vec![vec![Up, Right], vec![Right, Up]],
        (0, 4) => vec![vec![Up, Up, Left], vec![Up, Left, Up]],
        (0, 5) => vec![vec![Up, Up]],
        (0, 6) => vec![vec![Up, Up, Right], vec![Up, Right, Up], vec![
            Right, Up, Up,
        ]],
        (0, 7) => vec![vec![Up, Up, Up, Left], vec![Up, Up, Left, Up], vec![
            Up, Left, Up, Up,
        ]],
        (0, 8) => vec![vec![Up, Up, Up]],
        (0, 9) => vec![
            vec![Up, Up, Up, Right],
            vec![Up, Up, Right, Up],
            vec![Up, Right, Up, Up],
            vec![Right, Up, Up, Up],
        ],
        (0, 10) => vec![vec![Right]],
        (1, 0) => vec![vec![Right, Down]],
        (1, 1) => vec![vec![]],
        (1, 2) => vec![vec![Right]],
        (1, 3) => vec![vec![Right, Right]],
        (1, 4) => vec![vec![Up]],
        (1, 5) => vec![vec![Up, Right], vec![Right, Up]],
        (1, 6) => vec![vec![Up, Right, Right], vec![Right, Up, Right], vec![
            Right, Right, Up,
        ]],
        (1, 7) => vec![vec![Up, Up]],
        (1, 8) => vec![vec![Up, Up, Right], vec![Up, Right, Up], vec![
            Right, Up, Up,
        ]],
        (1, 9) => vec![
            vec![Up, Up, Right, Right],
            vec![Up, Right, Up, Right],
            vec![Up, Right, Right, Up],
            vec![Right, Up, Up, Right],
            vec![Right, Up, Right, Up],
            vec![Right, Right, Up, Up],
        ],
        (1, 10) => vec![vec![Right, Down, Right], vec![Right, Right, Down]],
        (2, 0) => vec![vec![Down]],
        (2, 1) => vec![vec![Left]],
        (2, 2) => vec![vec![]],
        (2, 3) => vec![vec![Right]],
        (2, 4) => vec![vec![Up, Left], vec![Left, Up]],
        (2, 5) => vec![vec![Up]],
        (2, 6) => vec![vec![Up, Right], vec![Right, Up]],
        (2, 7) => vec![vec![Up, Up, Left], vec![Up, Left, Up], vec![Left, Up, Up]],
        (2, 8) => vec![vec![Up, Up]],
        (2, 9) => vec![vec![Up, Up, Right], vec![Up, Right, Up], vec![
            Right, Up, Up,
        ]],
        (2, 10) => vec![vec![Down, Right], vec![Right, Down]],
        (3, 0) => vec![vec![Down, Left], vec![Left, Down]],
        (3, 1) => vec![vec![Left, Left]],
        (3, 2) => vec![vec![Left]],
        (3, 3) => vec![vec![]],
        (3, 4) => vec![vec![Up, Left, Left], vec![Left, Up, Left], vec![
            Left, Left, Up,
        ]],
        (3, 5) => vec![vec![Up, Left], vec![Left, Up]],
        (3, 6) => vec![vec![Up]],
        (3, 7) => vec![
            vec![Up, Up, Left, Left],
            vec![Up, Left, Up, Left],
            vec![Up, Left, Left, Up],
            vec![Left, Up, Up, Left],
            vec![Left, Up, Left, Up],
            vec![Left, Left, Up, Up],
        ],
        (3, 8) => vec![vec![Up, Up, Left], vec![Up, Left, Up], vec![Left, Up, Up]],
        (3, 9) => vec![vec![Up, Up]],
        (3, 10) => vec![vec![Down]],
        (4, 0) => vec![vec![Down, Right, Down], vec![Right, Down, Down]],
        (4, 1) => vec![vec![Down]],
        (4, 2) => vec![vec![Down, Right], vec![Right, Down]],
        (4, 3) => vec![vec![Down, Right, Right], vec![Right, Down, Right], vec![
            Right, Right, Down,
        ]],
        (4, 4) => vec![vec![]],
        (4, 5) => vec![vec![Right]],
        (4, 6) => vec![vec![Right, Right]],
        (4, 7) => vec![vec![Up]],
        (4, 8) => vec![vec![Up, Right], vec![Right, Up]],
        (4, 9) => vec![vec![Up, Right, Right], vec![Right, Up, Right], vec![
            Right, Right, Up,
        ]],
        (4, 10) => vec![
            vec![Down, Right, Down, Right],
            vec![Down, Right, Right, Down],
            vec![Right, Down, Down, Right],
            vec![Right, Down, Right, Down],
            vec![Right, Right, Down, Down],
        ],
        (5, 0) => vec![vec![Down, Down]],
        (5, 1) => vec![vec![Down, Left], vec![Left, Down]],
        (5, 2) => vec![vec![Down]],
        (5, 3) => vec![vec![Down, Right], vec![Right, Down]],
        (5, 4) => vec![vec![Left]],
        (5, 5) => vec![vec![]],
        (5, 6) => vec![vec![Right]],
        (5, 7) => vec![vec![Up, Left], vec![Left, Up]],
        (5, 8) => vec![vec![Up]],
        (5, 9) => vec![vec![Up, Right], vec![Right, Up]],
        (5, 10) => vec![vec![Down, Down, Right], vec![Down, Right, Down], vec![
            Right, Down, Down,
        ]],
        (6, 0) => vec![vec![Down, Down, Left], vec![Down, Left, Down], vec![
            Left, Down, Down,
        ]],
        (6, 1) => vec![vec![Down, Left, Left], vec![Left, Down, Left], vec![
            Left, Left, Down,
        ]],
        (6, 2) => vec![vec![Down, Left], vec![Left, Down]],
        (6, 3) => vec![vec![Down]],
        (6, 4) => vec![vec![Left, Left]],
        (6, 5) => vec![vec![Left]],
        (6, 6) => vec![vec![]],
        (6, 7) => vec![vec![Up, Left, Left], vec![Left, Up, Left], vec![
            Left, Left, Up,
        ]],
        (6, 8) => vec![vec![Up, Left], vec![Left, Up]],
        (6, 9) => vec![vec![Up]],
        (6, 10) => vec![vec![Down, Down]],
        (7, 0) => vec![
            vec![Down, Down, Right, Down],
            vec![Down, Right, Down, Down],
            vec![Right, Down, Down, Down],
        ],
        (7, 1) => vec![vec![Down, Down]],
        (7, 2) => vec![vec![Down, Down, Right], vec![Down, Right, Down], vec![
            Right, Down, Down,
        ]],
        (7, 3) => vec![
            vec![Down, Down, Right, Right],
            vec![Down, Right, Down, Right],
            vec![Down, Right, Right, Down],
            vec![Right, Down, Down, Right],
            vec![Right, Down, Right, Down],
            vec![Right, Right, Down, Down],
        ],
        (7, 4) => vec![vec![Down]],
        (7, 5) => vec![vec![Down, Right], vec![Right, Down]],
        (7, 6) => vec![vec![Down, Right, Right], vec![Right, Down, Right], vec![
            Right, Right, Down,
        ]],
        (7, 7) => vec![vec![]],
        (7, 8) => vec![vec![Right]],
        (7, 9) => vec![vec![Right, Right]],
        (7, 10) => vec![
            vec![Down, Down, Right, Down, Right],
            vec![Down, Down, Right, Right, Down],
            vec![Down, Right, Down, Down, Right],
            vec![Down, Right, Down, Right, Down],
            vec![Down, Right, Right, Down, Down],
            vec![Right, Down, Down, Down, Right],
            vec![Right, Down, Down, Right, Down],
            vec![Right, Down, Right, Down, Down],
            vec![Right, Right, Down, Down, Down],
        ],
        (8, 0) => vec![vec![Down, Down, Down]],
        (8, 1) => vec![vec![Down, Down, Left], vec![Down, Left, Down], vec![
            Left, Down, Down,
        ]],
        (8, 2) => vec![vec![Down, Down]],
        (8, 3) => vec![vec![Down, Down, Right], vec![Down, Right, Down], vec![
            Right, Down, Down,
        ]],
        (8, 4) => vec![vec![Down, Left], vec![Left, Down]],
        (8, 5) => vec![vec![Down]],
        (8, 6) => vec![vec![Down, Right], vec![Right, Down]],
        (8, 7) => vec![vec![Left]],
        (8, 8) => vec![vec![]],
        (8, 9) => vec![vec![Right]],
        (8, 10) => vec![
            vec![Down, Down, Down, Right],
            vec![Down, Down, Right, Down],
            vec![Down, Right, Down, Down],
            vec![Right, Down, Down, Down],
        ],
        (9, 0) => vec![
            vec![Down, Down, Down, Left],
            vec![Down, Down, Left, Down],
            vec![Down, Left, Down, Down],
            vec![Left, Down, Down, Down],
        ],
        (9, 1) => vec![
            vec![Down, Down, Left, Left],
            vec![Down, Left, Down, Left],
            vec![Down, Left, Left, Down],
            vec![Left, Down, Down, Left],
            vec![Left, Down, Left, Down],
            vec![Left, Left, Down, Down],
        ],
        (9, 2) => vec![vec![Down, Down, Left], vec![Down, Left, Down], vec![
            Left, Down, Down,
        ]],
        (9, 3) => vec![vec![Down, Down]],
        (9, 4) => vec![vec![Down, Left, Left], vec![Left, Down, Left], vec![
            Left, Left, Down,
        ]],
        (9, 5) => vec![vec![Down, Left], vec![Left, Down]],
        (9, 6) => vec![vec![Down]],
        (9, 7) => vec![vec![Left, Left]],
        (9, 8) => vec![vec![Left]],
        (9, 9) => vec![vec![]],
        (9, 10) => vec![vec![Down, Down, Down]],
        (10, 0) => vec![vec![Left]],
        (10, 1) => vec![vec![Up, Left, Left], vec![Left, Up, Left]],
        (10, 2) => vec![vec![Up, Left], vec![Left, Up]],
        (10, 3) => vec![vec![Up]],
        (10, 4) => vec![
            vec![Up, Up, Left, Left],
            vec![Up, Left, Up, Left],
            vec![Up, Left, Left, Up],
            vec![Left, Up, Up, Left],
            vec![Left, Up, Left, Up],
        ],
        (10, 5) => vec![vec![Up, Up, Left], vec![Up, Left, Up], vec![Left, Up, Up]],
        (10, 6) => vec![vec![Up, Up]],
        (10, 7) => vec![
            vec![Up, Up, Up, Left, Left],
            vec![Up, Up, Left, Up, Left],
            vec![Up, Up, Left, Left, Up],
            vec![Up, Left, Up, Up, Left],
            vec![Up, Left, Up, Left, Up],
            vec![Up, Left, Left, Up, Up],
            vec![Left, Up, Up, Up, Left],
            vec![Left, Up, Up, Left, Up],
            vec![Left, Up, Left, Up, Up],
        ],
        (10, 8) => vec![
            vec![Up, Up, Up, Left],
            vec![Up, Up, Left, Up],
            vec![Up, Left, Up, Up],
            vec![Left, Up, Up, Up],
        ],
        (10, 9) => vec![vec![Up, Up, Up]],
        (10, 10) => vec![vec![]],
        _ => unreachable!(),
    }
}
