use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

pub struct Input {
    start: String,
    rules: Vec<(u8, u8, u8)>,
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Input {
    let mut lines = input.lines();
    let start = lines.next().unwrap().to_owned();

    lines.next();

    let rules = lines
        .map(|line| {
            let mut b = line.bytes();
            let first = b.next().unwrap();
            let second = b.next().unwrap();

            let result = b.nth(4).unwrap();

            (first, second, result)
        })
        .collect();

    Input { start, rules }
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> u32 {
    let mut polymer = input.start.to_owned();

    for _ in 0..10 {
        let mut indices = vec![];

        for (i, w) in polymer.as_bytes().windows(2).enumerate() {
            for rule in &input.rules {
                if rule.0 == w[0] && rule.1 == w[1] {
                    indices.push((i + 1 + indices.len(), rule.2));
                }
            }
        }

        for (index, c) in indices {
            polymer.insert(index, c as char);
        }
    }

    let mut m: HashMap<u8, u32> = HashMap::new();
    for b in polymer.bytes() {
        *m.entry(b).or_default() += 1;
    }

    let max = m.values().copied().max().unwrap();
    let min = m.values().copied().min().unwrap();

    max - min
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> u64 {
    let mut first_cache = HashMap::new();
    for (first_char, second_char, _) in &input.rules {
        let mut polymer = String::new();
        polymer.push(*first_char as char);
        polymer.push(*second_char as char);

        for _ in 0..20 {
            let mut indices = vec![];

            for (i, w) in polymer.as_bytes().windows(2).enumerate() {
                for rule in &input.rules {
                    if rule.0 == w[0] && rule.1 == w[1] {
                        indices.push((i + 1 + indices.len(), rule.2));
                    }
                }
            }

            for (index, c) in indices {
                polymer.insert(index, c as char);
            }
        }

        let mut pairs: HashMap<(u8, u8), u64> = HashMap::new();
        for w in polymer.as_bytes().windows(2) {
            *pairs.entry((w[0], w[1])).or_default() += 1;
        }

        let mut chars: HashMap<u8, u64> = HashMap::new();
        let mut iter = polymer.bytes().skip(1).peekable();
        while let Some(b) = iter.next() {
            if let Some(_) = iter.peek() {
                *chars.entry(b).or_default() += 1;
            }
        }

        first_cache.insert((*first_char, *second_char), (pairs, chars));
    }

    let mut second_cache = HashMap::new();
    for (first_char, second_char, _) in &input.rules {
        let (cached_pairs, chars) = first_cache.get(&(*first_char, *second_char)).unwrap();

        let mut chars = chars.clone();

        for (pair, occurrences) in cached_pairs {
            let (_, cached_chars) = first_cache.get(pair).unwrap();

            for (c, n) in cached_chars {
                *chars.entry(*c).or_default() += *n * *occurrences;
            }
        }

        second_cache.insert((*first_char, *second_char), chars);
    }

    let mut total: HashMap<u8, u64> = HashMap::new();
    for w in input.start.as_bytes().windows(2) {
        if let Some(chars) = second_cache.get(&(w[0], w[1])) {
            for (c, n) in chars {
                *total.entry(*c).or_default() += *n;
            }
        }
    }
    for c in input.start.bytes() {
        *total.entry(c).or_default() += 1;
    }

    let min = total.values().copied().min().unwrap();
    let max = total.values().copied().max().unwrap();

    max - min
}
