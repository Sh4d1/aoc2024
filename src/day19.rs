use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Clone, Debug)]
pub struct Input {
    stripes: FxHashSet<String>,
    patterns: Vec<String>,
    max_stripe_len: usize,
}

#[aoc_generator(day19)]
pub fn parse(input: &str) -> Input {
    let (f, s) = input.split_once("\n\n").unwrap();
    let mut max = 0;
    let stripes = f
        .split(", ")
        .map(|s| {
            max = std::cmp::max(max, s.len());
            s.to_owned()
        })
        .collect();
    Input {
        max_stripe_len: max,
        stripes,
        patterns: s.lines().map(|s| s.to_owned()).collect(),
    }
}

fn solve(p: String, input: &Input, seen: &mut FxHashMap<String, usize>) -> usize {
    if p.is_empty() {
        return 1;
    }
    let mut res = 0;
    for i in 0..input.max_stripe_len {
        if i >= p.len() {
            break;
        }
        if let Some(s) = input.stripes.get(&p[0..=i]) {
            let np = p.strip_prefix(s).unwrap();
            res += if let Some(c) = seen.get(np) {
                *c
            } else {
                solve(np.to_owned(), input, seen)
            };
        }
    }
    seen.insert(p, res);
    res
}

#[aoc(day19, part1)]
pub fn part1(input: &Input) -> usize {
    input
        .patterns
        .iter()
        .filter(|&p| solve(p.to_owned(), input, &mut FxHashMap::default()) > 0)
        .count()
}

#[aoc(day19, part2)]
pub fn part2(input: &Input) -> usize {
    input.patterns.iter().fold(0, |acc, p| {
        acc + solve(p.clone().to_owned(), input, &mut FxHashMap::default())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    #[test]
    fn example1() {
        assert_eq!(part1(&parse(INPUT)), 6)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&parse(INPUT)), 16)
    }
}
