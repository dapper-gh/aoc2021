use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    let lines = input.lines();
    lines
        .map(|line| {
            let mut sides = line.split(" -> ").map(|side| {
                let mut nums = side.split(',').map(|point| point.parse().unwrap());
                (nums.next().unwrap(), nums.next().unwrap())
            });
            (sides.next().unwrap(), sides.next().unwrap())
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[((u32, u32), (u32, u32))]) -> u32 {
    let hv_lines = input
        .iter()
        .copied()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2);

    let mut points = [0u8; 1000 * 1000];

    let mut result = 0;
    for line in hv_lines {
        for x in u32::min(line.0 .0, line.1 .0)..=u32::max(line.0 .0, line.1 .0) {
            for y in u32::min(line.0 .1, line.1 .1)..=u32::max(line.0 .1, line.1 .1) {
                points[(x * 1000 + y) as usize] += 1;
                if points[(x * 1000 + y) as usize] == 2 {
                    result += 1;
                }
            }
        }
    }

    result
}

#[aoc(day5, part2)]
pub fn part2(input: &[((u32, u32), (u32, u32))]) -> u32 {
    let (hv_lines, diag_lines) = input
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2);

    let mut points = [0u8; 1000 * 1000];

    let mut result = 0;
    for line in hv_lines {
        for x in u32::min(line.0 .0, line.1 .0)..=u32::max(line.0 .0, line.1 .0) {
            for y in u32::min(line.0 .1, line.1 .1)..=u32::max(line.0 .1, line.1 .1) {
                points[(x * 1000 + y) as usize] += 1;
                if points[(x * 1000 + y) as usize] == 2 {
                    result += 1;
                }
            }
        }
    }

    for line in diag_lines {
        let ((x1, y1), (x2, y2)) = line;

        for i in 0..=(u32::max(x1, x2) - u32::min(x1, x2)) {
            let x = if x1 > x2 { x1 - i } else { x1 + i };
            let y = if y1 > y2 { y1 - i } else { y1 + i };

            points[(x * 1000 + y) as usize] += 1;
            if points[(x * 1000 + y) as usize] == 2 {
                result += 1;
            }
        }
    }

    result
}
