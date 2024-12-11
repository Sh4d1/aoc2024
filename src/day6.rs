use bitflags::bitflags;
use rustc_hash::FxHashSet as HashSet;

bitflags! {
   #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
   struct GuardDirections: u8 {
       const UP = 1;
       const DOWN = 1 << 1;
       const LEFT = 1 << 2;
       const RIGHT = 1 << 3;
   }
}

impl From<Direction> for GuardDirections {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => GuardDirections::UP,
            Direction::Down => GuardDirections::DOWN,
            Direction::Left => GuardDirections::LEFT,
            Direction::Right => GuardDirections::RIGHT,
        }
    }
}

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
    let mut visited = vec![vec![GuardDirections::empty(); input.w]; input.h];

    let mut guard = input.guard_start;
    visited[guard.0][guard.1] = guard.2.into();
    let mut res = 1;

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
        if visited[guard.0][guard.1].contains(guard.2.into()) {
            break;
        }

        if visited[guard.0][guard.1].is_empty() {
            res += 1;
        }
        visited[guard.0][guard.1].insert(guard.2.into());
    }
    res
}

#[aoc(day6, part2)]
pub fn part2(input: &Input) -> usize {
    let mut visited = vec![vec![GuardDirections::empty(); input.w]; input.h];

    let mut guard = input.guard_start;
    visited[guard.0][guard.1] = guard.2.into();
    let mut res = HashSet::default();

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
            let new_block = next_pos;

            if visited[new_block.0][new_block.1].is_empty() {
                let mut new_guard = (guard.0, guard.1, guard.2.rotate());
                let mut new_visited = visited.clone();
                new_visited[new_guard.0][new_guard.1] = new_guard.2.into();
                new_visited[guard.0][guard.1] = guard.2.into();
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
                    if new_visited[new_guard.0][new_guard.1].contains(new_guard.2.into()) {
                        res.insert(new_block);
                        break;
                    }
                    new_visited[new_guard.0][new_guard.1].insert(new_guard.2.into());
                }
            }

            guard = (next_pos.0, next_pos.1, guard.2);
        }

        if visited[guard.0][guard.1].contains(guard.2.into()) {
            break;
        }
        visited[guard.0][guard.1] = guard.2.into();
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
