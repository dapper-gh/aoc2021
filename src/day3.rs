use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> String {
    input.to_owned()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let mut bits = [0u32; 12];

    let mut i = 0;
    for byte in input.bytes() {
        if i == 12 {
            i = 0;
        } else {
            bits[i] += byte as u32;
            i += 1;
        }
    }

    let out = bits
        .into_iter()
        .fold(0, |a, n| (a * 2) + ((n >= (48 * 1000 + 500)) as u32));

    out * (!out & 0b1111_1111_1111)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let mut remaining = lines.clone();

    let mut i = 0;
    while remaining.len() > 1 {
        let mut to_remove = Vec::with_capacity(16);

        let bits: usize = remaining
            .iter()
            .map(|s| s.bytes().nth(i).unwrap() as usize)
            .sum();
        let expected = if bits >= (48 * remaining.len() + (remaining.len() + 1) / 2) {
            b'1'
        } else {
            b'0'
        };

        let mut index = 0;
        for item in &remaining {
            if item.bytes().nth(i) != Some(expected) {
                to_remove.push(index);
            } else {
                index += 1;
            }
        }

        for index in to_remove {
            remaining.remove(index);
        }

        i += 1;
    }

    let oxy = remaining
        .get(0)
        .unwrap()
        .bytes()
        .fold(0, |a, n| (a * 2) + ((n - 48) as u32));

    let mut remaining = lines.clone();

    let mut i = 0;
    while remaining.len() > 1 {
        let mut to_remove = Vec::with_capacity(16);

        let bits: usize = remaining
            .iter()
            .map(|s| s.bytes().nth(i).unwrap() as usize)
            .sum();
        let expected = if bits >= (48 * remaining.len() + (remaining.len() + 1) / 2) {
            b'0'
        } else {
            b'1'
        };

        let mut index = 0;
        for item in &remaining {
            if item.bytes().nth(i) != Some(expected) {
                to_remove.push(index);
            } else {
                index += 1;
            }
        }

        for index in to_remove {
            remaining.remove(index);
        }

        i += 1;
    }

    let co2 = remaining
        .get(0)
        .unwrap()
        .bytes()
        .fold(0, |a, n| (a * 2) + ((n - 48) as u32));

    oxy * co2
}
