use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    pub fn rotate(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn all() -> [Self; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Used,
    Guard(Direction),
}

#[derive(Debug, Clone)]
pub struct Input {
    grid: Vec<Vec<Cell>>,
    w: usize,
    h: usize,
    guard_start: (usize, usize, Direction),
}

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Input {
    let mut guard = None;
    let grid: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Used,
                    _ => {
                        let direction = match c {
                            '^' => Direction::Up,
                            '>' => Direction::Right,
                            '<' => Direction::Left,
                            'v' => Direction::Down,
                            _ => unreachable!(),
                        };
                        guard = Some((i, j, direction));
                        Cell::Guard(direction)
                    }
                })
                .collect()
        })
        .collect();
    Input {
        w: grid[0].len(),
        h: grid.len(),
        grid,
        guard_start: guard.unwrap(),
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &Input) -> usize {
    let mut visited = HashSet::new();
    let mut visited_no_dir = HashSet::new();
    let mut guard = input.guard_start;
    visited.insert(guard);
    visited_no_dir.insert((guard.0, guard.1));
    loop {
        let next_pos = (
            guard.0.wrapping_add_signed(guard.2.get_delta().0),
            guard.1.wrapping_add_signed(guard.2.get_delta().1),
        );
        if next_pos.0 >= input.h || next_pos.1 >= input.w {
            break;
        }
        if let Cell::Used = input.grid[next_pos.0][next_pos.1] {
            guard = (guard.0, guard.1, guard.2.rotate());
        } else {
            guard = (next_pos.0, next_pos.1, guard.2);
        }
        if visited.contains(&guard) {
            break;
        }
        visited.insert(guard);
        visited_no_dir.insert((guard.0, guard.1));
    }
    visited_no_dir.len()
}

#[aoc(day6, part2)]
pub fn part2(input: &Input) -> usize {
    let mut visited = HashSet::new();
    let mut guard = input.guard_start;
    let mut res = HashSet::new();
    visited.insert(guard);

    loop {
        let (dx, dy) = guard.2.get_delta();
        let next_pos = (
            guard.0.wrapping_add_signed(dx),
            guard.1.wrapping_add_signed(dy),
        );
        if next_pos.0 >= input.h || next_pos.1 >= input.w {
            break;
        }
        if let Cell::Used = input.grid[next_pos.0][next_pos.1] {
            guard = (guard.0, guard.1, guard.2.rotate());
        } else {
            let new_block = next_pos;

            if !Direction::all()
                .iter()
                .any(|d| visited.contains(&(new_block.0, new_block.1, *d)))
            {
                let mut new_guard = (guard.0, guard.1, guard.2.rotate());
                let mut new_visited = visited.clone();
                new_visited.insert(new_guard);
                new_visited.insert(guard);
                loop {
                    let new_guard_pos = (
                        new_guard.0.wrapping_add_signed(new_guard.2.get_delta().0),
                        new_guard.1.wrapping_add_signed(new_guard.2.get_delta().1),
                    );
                    if new_guard_pos.0 >= input.h || new_guard_pos.1 >= input.w {
                        break;
                    }
                    if input.grid[new_guard_pos.0][new_guard_pos.1] == Cell::Used
                        || new_guard_pos == new_block
                    {
                        new_guard = (new_guard.0, new_guard.1, new_guard.2.rotate());
                    } else {
                        new_guard = (new_guard_pos.0, new_guard_pos.1, new_guard.2);
                    }
                    if new_visited.contains(&new_guard) {
                        res.insert(new_block);
                        break;
                    }
                    new_visited.insert(new_guard);
                }
            }

            guard = (next_pos.0, next_pos.1, guard.2);
        }
        if visited.contains(&guard) {
            break;
        }
        visited.insert(guard);
    }
    res.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 41)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 6)
    }
}
