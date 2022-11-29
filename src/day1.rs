use crate::opt_parse;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u16> {
    opt_parse::parse_u16_lines(input)
}

#[aoc(day1, part1)]
pub fn part1(input: &[u16]) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u16]) -> usize {
    input.windows(4).filter(|w| w[3] > w[0]).count()
}
