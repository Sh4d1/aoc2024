#[derive(Debug, Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Right,
    Left,
}

impl Move {
    fn get_dir(&self) -> (isize, isize) {
        match self {
            Move::Down => (1, 0),
            Move::Up => (-1, 0),
            Move::Left => (0, -1),
            Move::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Cell {
    Empty,
    Box(bool),
    Wall,
}

#[derive(Debug, Clone)]
pub struct Input {
    grid: Vec<Vec<Cell>>,
    moves: Vec<Move>,
    robot_pos: (usize, usize),
}

#[aoc_generator(day15)]
pub fn parse(input: &str) -> Input {
    let (first, sec) = input.split_once("\n\n").unwrap();
    let mut robot_pos = (0, 0);
    let grid = first
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'O' => Cell::Box(false),
                    '@' => {
                        robot_pos = (i, j);
                        Cell::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    Input {
        grid,
        robot_pos,
        moves: sec
            .chars()
            .filter_map(|c| match c {
                '>' => Some(Move::Right),
                '<' => Some(Move::Left),
                '^' => Some(Move::Up),
                'v' => Some(Move::Down),
                _ => None,
            })
            .collect(),
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &Input) -> usize {
    let mut p = input.robot_pos;
    let mut grid = input.grid.clone();
    input.moves.iter().for_each(|m| {
        let dp = m.get_dir();
        let next = (p.0.wrapping_add_signed(dp.0), p.1.wrapping_add_signed(dp.1));
        match grid[next.0][next.1] {
            Cell::Empty => p = next,
            Cell::Wall => (),
            Cell::Box(_) => {
                let mut tmp = next;
                while let Cell::Box(_) = grid[tmp.0][tmp.1] {
                    tmp = (
                        tmp.0.wrapping_add_signed(dp.0),
                        tmp.1.wrapping_add_signed(dp.1),
                    );
                }
                if let Cell::Empty = grid[tmp.0][tmp.1] {
                    grid[tmp.0][tmp.1] = Cell::Box(false);
                    grid[next.0][next.1] = Cell::Empty;
                    p = next;
                }
            }
        }
    });
    grid.iter().enumerate().fold(0, |acc, (i, r)| {
        acc + r.iter().enumerate().fold(0, |acci, (j, c)| match c {
            Cell::Box(_) => acci + 100 * i + j,
            _ => acci,
        })
    })
}

fn move_box(grid: &mut Vec<Vec<Cell>>, p_box: (usize, usize), dir: (isize, isize), is_left: bool) {
    let next = (
        p_box.0.wrapping_add_signed(dir.0),
        p_box.1.wrapping_add_signed(dir.1),
    );

    match grid[next.0][next.1] {
        Cell::Empty => grid[next.0][next.1] = Cell::Box(is_left),
        Cell::Box(is_inner_left) => {
            if dir.1 != 0 {
                move_box(grid, next, dir, is_inner_left);
                grid[next.0][next.1] = Cell::Box(!is_inner_left);
                return;
            }
            if is_inner_left == is_left {
                move_box(grid, next, dir, is_left);
                grid[next.0][next.1] = Cell::Box(is_inner_left);
                return;
            }

            move_box(grid, next, dir, is_inner_left);
            if is_inner_left {
                move_box(grid, (next.0, next.1 + 1), dir, !is_inner_left);
                grid[next.0][next.1 + 1] = Cell::Empty;
            } else {
                move_box(grid, (next.0, next.1 - 1), dir, !is_inner_left);
                grid[next.0][next.1 - 1] = Cell::Empty;
            }
            grid[next.0][next.1] = Cell::Box(is_left);
        }
        _ => unreachable!(),
    }
}
fn can_move_box(
    grid: &mut Vec<Vec<Cell>>,
    p_box: (usize, usize),
    dir: (isize, isize),
    is_left: bool,
) -> bool {
    let next = (
        p_box.0.wrapping_add_signed(dir.0),
        p_box.1.wrapping_add_signed(dir.1),
    );

    match grid[next.0][next.1] {
        Cell::Empty => true,
        Cell::Box(is_inner_left) => {
            if is_inner_left == is_left || dir.1 != 0 {
                return can_move_box(grid, next, dir, is_inner_left);
            }
            let first = can_move_box(grid, next, dir, is_inner_left);
            if !first {
                return false;
            }
            if is_inner_left {
                can_move_box(grid, (next.0, next.1 + 1), dir, !is_inner_left)
            } else {
                can_move_box(grid, (next.0, next.1 - 1), dir, !is_inner_left)
            }
        }
        Cell::Wall => false,
    }
}

#[aoc(day15, part2)]
pub fn part2(input: &Input) -> usize {
    let mut pos = (input.robot_pos.0, input.robot_pos.1 * 2);
    let mut grid = input.grid.iter().fold(Vec::new(), |mut acc, r| {
        acc.push(r.iter().fold(
            Vec::with_capacity(input.grid[0].len() * 2),
            |mut acci, c| {
                match c {
                    Cell::Empty => {
                        acci.push(Cell::Empty);
                        acci.push(Cell::Empty);
                    }
                    Cell::Box(_) => {
                        acci.push(Cell::Box(true));
                        acci.push(Cell::Box(false));
                    }
                    Cell::Wall => {
                        acci.push(Cell::Wall);
                        acci.push(Cell::Wall);
                    }
                }
                acci
            },
        ));
        acc
    });
    input.moves.iter().for_each(|m| {
        let dp = m.get_dir();
        let next = (
            pos.0.wrapping_add_signed(dp.0),
            pos.1.wrapping_add_signed(dp.1),
        );

        match grid[next.0][next.1] {
            Cell::Empty => pos = next,
            Cell::Box(is_left) => {
                let other = if is_left {
                    (next.0, next.1 + 1)
                } else {
                    (next.0, next.1 - 1)
                };
                if dp.1 != 0 {
                    if can_move_box(&mut grid, next, dp, is_left) {
                        move_box(&mut grid, next, dp, is_left);
                        grid[next.0][next.1] = Cell::Empty;
                        pos = next;
                    }
                } else {
                    if can_move_box(&mut grid, next, dp, is_left)
                        && can_move_box(&mut grid, other, dp, !is_left)
                    {
                        move_box(&mut grid, next, dp, is_left);
                        move_box(&mut grid, other, dp, !is_left);
                        grid[next.0][next.1] = Cell::Empty;
                        grid[other.0][other.1] = Cell::Empty;
                        pos = next;
                    }
                }
            }
            Cell::Wall => (),
        }
    });
    grid.iter().enumerate().fold(0, |acc, (i, r)| {
        acc + r.iter().enumerate().fold(0, |acci, (j, c)| match c {
            Cell::Box(true) => acci + 100 * i + j,
            _ => acci,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 10092)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 9021)
    }
}
