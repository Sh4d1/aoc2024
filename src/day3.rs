use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, u32},
    combinator::map,
    multi::fold_many0,
    sequence::{delimited, tuple},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Element {
    Mul(u32, u32),
    Do,
    Dont,
    Gibberish,
}

fn parse_mul(input: &str) -> IResult<&str, Element> {
    map(
        delimited(tag("mul("), tuple((u32, char(','), u32)), char(')')),
        |(a, _, b)| Element::Mul(a, b),
    )(input)
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<Element> {
    fold_many0(
        alt((
            parse_mul,
            map(tag("do()"), |_| Element::Do),
            map(tag("don't()"), |_| Element::Dont),
            map(take(1usize), |_| Element::Gibberish),
        )),
        Vec::new,
        |mut acc, elem| {
            if elem != Element::Gibberish {
                acc.push(elem);
            }
            acc
        },
    )(input)
    .unwrap()
    .1
}

#[aoc(day3, part1)]
pub fn part1(input: &[Element]) -> u32 {
    input.iter().fold(0, |acc, elem| {
        if let Element::Mul(a, b) = elem {
            acc + a * b
        } else {
            acc
        }
    })
}

#[aoc(day3, part2)]
pub fn part2(input: &[Element]) -> u32 {
    input
        .iter()
        .fold((0, true), |mut acc, elem| {
            match elem {
                Element::Mul(a, b) if acc.1 => acc.0 += a * b,
                Element::Do => acc.1 = true,
                Element::Dont => acc.1 = false,
                _ => (),
            }
            acc
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&parse(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )),
            161
        )
    }
    #[test]
    fn example2() {
        assert_eq!(
            part2(&parse(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            48
        )
    }
}
