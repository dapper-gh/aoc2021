use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<u8> {
    input.bytes().step_by(2).map(|b| b - 48).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[u8]) -> u64 {
    input
        .iter()
        .copied()
        .map(|input| match input {
            0 => 1421,
            1 => 1401,
            2 => 1191,
            3 => 1154,
            4 => 1034,
            5 => 950,
            6 => 905,

            _ => panic!("Invalid input"),
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &[u8]) -> u64 {
    input
        .iter()
        .copied()
        .map(|input| match input {
            0 => 6703087164,
            1 => 6206821033,
            2 => 5617089148,
            3 => 5217223242,
            4 => 4726100874,
            5 => 4368232009,
            6 => 3989468462,

            _ => panic!("Invalid input"),
        })
        .sum()
}
