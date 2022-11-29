use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<u8> {
    input.bytes().filter_map(|n| n.checked_sub(48)).collect()
}

static SIZE: usize = 100;

#[aoc(day9, part1)]
pub fn part1(input: &[u8]) -> u32 {
    input
        .iter()
        .copied()
        .enumerate()
        .filter(|(i, n)| {
            if (i % SIZE) != 0 && input.get(i - 1).map(|v| v <= n) == Some(true) {
                return false;
            }

            if (i % SIZE) != (SIZE - 1) && input.get(i + 1).map(|v| v <= n) == Some(true) {
                return false;
            }

            if input.get(i - SIZE).map(|v| v <= n) == Some(true) {
                return false;
            }

            input.get(i + SIZE).map(|v| v <= n) != Some(true)
        })
        .map(|(_, n)| (n + 1) as u32)
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &[u8]) -> usize {
    let mut basins: Vec<_> = input
        .iter()
        .copied()
        .enumerate()
        .filter(|(i, n)| {
            if (i % SIZE) != 0 && input.get(i - 1).map(|v| v <= n) == Some(true) {
                return false;
            }

            if (i % SIZE) != (SIZE - 1) && input.get(i + 1).map(|v| v <= n) == Some(true) {
                return false;
            }

            if input.get(i - SIZE).map(|v| v <= n) == Some(true) {
                return false;
            }

            input.get(i + SIZE).map(|v| v <= n) != Some(true)
        })
        .map(|(i, _n)| vec![i])
        .collect();

    let mut added = true;
    while added {
        added = false;

        for basin in &mut basins {
            for num in basin.clone() {
                if (num % SIZE) != 0
                    && input.get(num - 1).map(|v| *v > input[num] && *v != 9) == Some(true)
                    && !basin.contains(&(num - 1))
                {
                    basin.push(num - 1);
                    added = true;
                }

                if (num % SIZE) != (SIZE - 1)
                    && input.get(num + 1).map(|v| *v > input[num] && *v != 9) == Some(true)
                    && !basin.contains(&(num + 1))
                {
                    basin.push(num + 1);
                    added = true;
                }

                if input.get(num + SIZE).map(|v| *v > input[num] && *v != 9) == Some(true)
                    && !basin.contains(&(num + SIZE))
                {
                    basin.push(num + SIZE);
                    added = true;
                }

                if input.get(num - SIZE).map(|v| *v > input[num] && *v != 9) == Some(true)
                    && !basin.contains(&(num - SIZE))
                {
                    basin.push(num - SIZE);
                    added = true;
                }
            }
        }
    }

    basins.sort_unstable_by(|a, b| b.len().cmp(&a.len()));

    basins[0].len() * basins[1].len() * basins[2].len()
}
