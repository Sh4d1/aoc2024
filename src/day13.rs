#[derive(Debug, Clone)]
pub struct Game {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl Game {
    // let's do some old school maths
    fn solve(&self) -> isize {
        let k = (self.prize.1 * self.a.0 - self.prize.0 * self.a.1)
            / (self.b.1 * self.a.0 - self.b.0 * self.a.1);
        let j = (self.prize.0 - k * self.b.0) / self.a.0;
        if self.a.0 * j + self.b.0 * k != self.prize.0
            || self.a.1 * j + self.b.1 * k != self.prize.1
        {
            return 0;
        }
        j * 3 + k
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    games: Vec<Game>,
}

#[aoc_generator(day13)]
pub fn parse(input: &str) -> Input {
    let games = input
        .split("\n\n")
        .map(|ll| {
            let mut lines = ll.lines();
            let a = lines.next().unwrap().strip_prefix("Button A: X+").unwrap();
            let b = lines.next().unwrap().strip_prefix("Button B: X+").unwrap();
            let prize = lines.next().unwrap().strip_prefix("Prize: X=").unwrap();

            let a = a.split_once(", Y+").unwrap();
            let b = b.split_once(", Y+").unwrap();
            let prize = prize.split_once(", Y=").unwrap();

            Game {
                a: (a.0.parse().unwrap(), a.1.parse().unwrap()),
                b: (b.0.parse().unwrap(), b.1.parse().unwrap()),
                prize: (prize.0.parse().unwrap(), prize.1.parse().unwrap()),
            }
        })
        .collect();

    Input { games }
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> isize {
    input.games.iter().fold(0, |acc, g| acc + g.solve())
}
#[aoc(day13, part2)]
pub fn part2(input: &Input) -> isize {
    input.games.iter().fold(0, |acc, g| {
        let mut g = g.clone();
        g.prize.0 += 10000000000000;
        g.prize.1 += 10000000000000;
        acc + g.solve()
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 480)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 0)
    }
}
