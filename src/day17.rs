use std::{collections::VecDeque, ops::BitXor};

pub enum Inst {
    Adv(usize), // A / 2^combo -> A (truncatd)
    Bxl(usize), // B xor literal
    Bst(usize), // combo % 8 -> B
    Jnz(usize), // if A != 0 => jump litteral
    Bxc(usize), // B bit xor C (ignore the operand)
    Out(usize), // combo % mod 8 => output
    Bdv(usize), // B / 2^combo -> A (truncated)
    Cdv(usize), // C / 2^combo -> A (truncated)
}
pub struct Input {
    a: usize,
    _b: usize,
    _c: usize,

    inst: Vec<Inst>,
    input: Vec<usize>,
}
#[aoc_generator(day17)]
pub fn parse(input: &str) -> Input {
    let (regs, instructions) = input.split_once("\n\n").unwrap();
    let mut reg_it = regs.lines();

    let int_inst: Vec<usize> = instructions
        .lines()
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();

    let mut i = 0;
    let mut inst = Vec::new();
    loop {
        if i >= int_inst.len() {
            break;
        }
        match int_inst[i] {
            0 => inst.push(Inst::Adv(int_inst[i + 1])),
            1 => inst.push(Inst::Bxl(int_inst[i + 1])),
            2 => inst.push(Inst::Bst(int_inst[i + 1])),
            3 => inst.push(Inst::Jnz(int_inst[i + 1])),
            4 => inst.push(Inst::Bxc(int_inst[i + 1])),
            5 => inst.push(Inst::Out(int_inst[i + 1])),
            6 => inst.push(Inst::Bdv(int_inst[i + 1])),
            7 => inst.push(Inst::Cdv(int_inst[i + 1])),
            _ => unreachable!(),
        }
        i += 2;
    }

    Input {
        a: reg_it
            .next()
            .unwrap()
            .strip_prefix("Register A: ")
            .unwrap()
            .parse()
            .unwrap(),
        _b: reg_it
            .next()
            .unwrap()
            .strip_prefix("Register B: ")
            .unwrap()
            .parse()
            .unwrap(),
        _c: reg_it
            .next()
            .unwrap()
            .strip_prefix("Register C: ")
            .unwrap()
            .parse()
            .unwrap(),
        inst,
        input: int_inst,
    }
}

fn get_combo(combo: usize, a: usize, b: usize, c: usize) -> usize {
    match combo {
        0..=3 => combo,
        4 => a,
        5 => b,
        6 => c,
        7.. => unreachable!(),
    }
}

fn run(insts: &Vec<Inst>, a: usize) -> Vec<usize> {
    let (mut a, mut b, mut c) = (a, 0, 0);
    let mut pc = 0;
    let mut res = Vec::new();
    loop {
        if pc >= insts.len() {
            break;
        }
        match insts[pc] {
            Inst::Adv(combo) => {
                a = a / 2usize.pow(get_combo(combo, a, b, c) as u32);
                pc += 1;
            }
            Inst::Bxl(lit) => {
                b = b.bitxor(lit);
                pc += 1;
            }
            Inst::Bst(combo) => {
                b = get_combo(combo, a, b, c) % 8;
                pc += 1;
            }
            Inst::Jnz(lit) => {
                if a != 0 {
                    pc = lit;
                } else {
                    pc += 1;
                }
            }
            Inst::Bxc(_) => {
                b = b.bitxor(c);
                pc += 1;
            }
            Inst::Out(combo) => {
                res.push(get_combo(combo, a, b, c) % 8);
                pc += 1;
            }
            Inst::Bdv(combo) => {
                b = a / 2usize.pow(get_combo(combo, a, b, c) as u32);
                pc += 1;
            }
            Inst::Cdv(combo) => {
                c = a / 2usize.pow(get_combo(combo, a, b, c) as u32);
                pc += 1;
            }
        }
    }
    res
}

#[aoc(day17, part1)]
pub fn part1(input: &Input) -> usize {
    run(&input.inst, input.a)
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, r)| acc + 10usize.pow(i as u32) * r)
}
#[aoc(day17, part2)]
pub fn part2(input: &Input) -> usize {
    let mut tests = VecDeque::new();
    tests.push_front((0, input.input.len() - 1));

    while let Some((a, shift)) = tests.pop_front() {
        for v in 0..8 {
            let new_a = (a << 3) + v;
            let res = run(&input.inst, new_a);
            let common = input
                .input
                .iter()
                .skip(shift)
                .enumerate()
                .all(|(i, c)| i < res.len() && res[i] == *c);

            if common {
                if shift == 0 {
                    return new_a;
                }
                tests.push_back((new_a, shift - 1));
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 4635635210)
    }
}
