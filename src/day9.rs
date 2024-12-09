#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    File(usize, usize),
    Free(usize),
}

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Vec<Cell> {
    input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                Cell::File(i / 2, c.to_digit(10).unwrap() as usize)
            } else {
                Cell::Free(c.to_digit(10).unwrap() as usize)
            }
        })
        .collect()
}
#[aoc(day9, part1)]
pub fn part1(input: &[Cell]) -> usize {
    let mut input = input.to_vec();
    let mut i = 0;
    loop {
        if i >= input.len() {
            break;
        }
        match input[i] {
            Cell::File(_, _) => {
                i += 1;
            }
            Cell::Free(free_size) => {
                if let Some(last) = input.pop() {
                    match last {
                        Cell::File(id, size) => {
                            if size <= free_size {
                                input.insert(i, last);
                                if size != free_size {
                                    input[i + 1] = Cell::Free(free_size - size);
                                } else {
                                    input.remove(i + 1);
                                    i += 1;
                                }
                            } else {
                                input.remove(i);
                                input.insert(i, Cell::File(id, free_size));
                                input.push(Cell::File(id, size - free_size));
                                i += 1;
                            }
                        }
                        Cell::Free(_) => (),
                    }
                } else {
                    break;
                    // unreachable!();
                }
            }
        }
    }
    let mut res = 0;
    let mut i = 0;
    input.iter().for_each(|c| match c {
        Cell::File(id, size) => {
            for _ in 0..*size {
                res += id * i;
                i += 1;
            }
        }
        Cell::Free(_) => unreachable!(),
    });
    res
}
#[aoc(day9, part2)]
pub fn part2(input: &[Cell]) -> usize {
    let mut input = input.to_vec();
    let mut i = input.len() - 1;

    loop {
        if i == 0 {
            break;
        }
        match input[i] {
            Cell::File(id, size) => {
                let mut j = 0;
                loop {
                    if j >= input.len() || j >= i {
                        break;
                    }
                    match input[j] {
                        Cell::File(_, _) => (),
                        Cell::Free(free_size) => {
                            if free_size == size {
                                input[j] = Cell::File(id, size);
                                input[i] = Cell::Free(size);
                                break;
                            } else if free_size > size {
                                input[j] = Cell::Free(free_size - size);
                                input[i] = Cell::Free(size);
                                input.insert(j, Cell::File(id, size));
                                if j <= i {
                                    i += 2;
                                }
                                break;
                            }
                        }
                    }
                    j += 1;
                }
            }
            Cell::Free(_) => (),
        }
        i -= 1;
    }

    let mut res = 0;
    let mut i = 0;
    input.iter().for_each(|c| match c {
        Cell::File(id, size) => {
            for _ in 0..*size {
                res += id * i;
                i += 1;
            }
        }
        Cell::Free(size) => i += size,
    });
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2333133121414131402";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 1928)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 2858)
    }
}
