use crate::opt_parse;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone)]
enum Line {
    Vertical(u16),
    Horizontal(u16),
}

#[derive(Default)]
pub struct Input {
    instructions: Vec<Line>,
    points: Vec<(u16, u16)>,
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Input {
    input.lines().filter(|line| !line.trim().is_empty()).fold(
        Input::default(),
        |mut input, line| {
            let nums = opt_parse::parse_u16_lines(line);

            match nums.get(1) {
                Some(y) => {
                    input.points.push((nums[0], *y));
                }
                None => input.instructions.push(match line.bytes().nth(11) {
                    Some(b'x') => Line::Vertical(nums[0]),
                    Some(b'y') => Line::Horizontal(nums[0]),

                    _ => panic!("Invalid fold direction"),
                }),
            };

            input
        },
    )
}

fn reflect_coord(val: u16, over: u16) -> u16 {
    if over > val {
        val
    } else {
        (over * 2) - val
    }
}

fn reflect((x, y): (u16, u16), fold: Line) -> (u16, u16) {
    match fold {
        Line::Horizontal(over) => (x, reflect_coord(y, over)),
        Line::Vertical(over) => (reflect_coord(x, over), y),
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> usize {
    let line = input.instructions[0];

    let mut points: Vec<_> = input
        .points
        .iter()
        .copied()
        .map(|point| reflect(point, line))
        .collect();

    points.sort_unstable();
    points.dedup();

    points.len()
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> String {
    let mut points: Vec<_> = input
        .points
        .iter()
        .copied()
        .map(|mut point| {
            for line in input.instructions.iter().copied() {
                point = reflect(point, line);
            }

            point
        })
        .collect();

    points.sort_unstable();
    points.dedup();

    let (max_x, _) = points
        .iter()
        .copied()
        .max_by_key(|(x, _)| *x)
        .expect("No points");
    let (_, max_y) = points
        .iter()
        .copied()
        .max_by_key(|(_, y)| *y)
        .expect("No points");

    let mut ans = String::with_capacity(((max_x + 1) * max_y) as usize);
    ans.push('\n');
    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                ans.push('#');
            } else {
                ans.push('.');
            }
        }
        ans.push('\n');
    }

    ans
}
