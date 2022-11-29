use crate::opt_parse;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<u16> {
    opt_parse::parse_u16_lines(input)
}

#[aoc(day7, part1)]
pub fn part1(input: &[u16]) -> u32 {
    let mut med = *input.iter().max().unwrap() as u32;

    let mut best = u32::MAX;

    while let Some(guess) = med.checked_sub(1) {
        let total = input
            .iter()
            .copied()
            .map(|u| u as u32)
            .map(|u| {
                u.checked_sub(guess)
                    .or_else(|| guess.checked_sub(u))
                    .unwrap()
            })
            .sum();
        if best > total {
            best = total;
        }

        med = guess;
    }

    best
}

fn pyramid(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

#[aoc(day7, part2)]
pub fn part2(input: &[u16]) -> u32 {
    let mut med = *input.iter().max().unwrap() as u32;

    let mut best = u32::MAX;

    while let Some(guess) = med.checked_sub(1) {
        let total = input
            .iter()
            .copied()
            .map(|u| u as u32)
            .map(|u| {
                let diff = u
                    .checked_sub(guess)
                    .or_else(|| guess.checked_sub(u))
                    .unwrap();

                pyramid(diff)
            })
            .sum();
        if best > total {
            best = total;
        }

        med = guess;
    }

    best
}
