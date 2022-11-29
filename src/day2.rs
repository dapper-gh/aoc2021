use aoc_runner_derive::{aoc, aoc_generator};

pub enum Movement {
    Horizontal(i32),
    Depth(i32),
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Movement> {
    let mut v = Vec::with_capacity(512);

    let mut bytes = input.bytes();
    while let Some(byte) = bytes.next() {
        match byte {
            b'u' => {
                v.push(Movement::Depth(
                    -(((bytes.nth(3).unwrap() as u8) - 48) as i32),
                ));
            }
            b'd' => {
                v.push(Movement::Depth(((bytes.nth(5).unwrap() as u8) - 48) as i32));
            }

            b'f' => {
                v.push(Movement::Horizontal(
                    ((bytes.nth(7).unwrap() as u8) - 48) as i32,
                ));
            }

            _ => {}
        }
    }

    v
}

#[aoc(day2, part1)]
pub fn part1(input: &[Movement]) -> i32 {
    let mut depth = 0;
    let mut horiz = 0;

    for mov in input {
        match mov {
            Movement::Depth(i) => depth += *i,
            Movement::Horizontal(i) => horiz += *i,
        }
    }

    depth * horiz
}

#[aoc(day2, part2)]
pub fn part2(input: &[Movement]) -> i32 {
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;

    for mov in input {
        match mov {
            Movement::Horizontal(i) => {
                depth += *i;
                horiz += aim * *i;
            }
            Movement::Depth(i) => aim += *i,
        }
    }

    depth * horiz
}
