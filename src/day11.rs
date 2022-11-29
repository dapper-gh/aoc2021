use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
pub enum State {
    Flashed(u8),
    Energy(u8),
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<State> {
    input
        .bytes()
        .filter_map(|n| n.checked_sub(48).map(State::Energy))
        .collect()
}

const SIZE: usize = 10;

fn flash_around<'a>(
    mut v: &'a mut [State],
    i: usize,
    round: u8,
    total: &mut u16,
) -> &'a mut [State] {
    if (i % SIZE) != 0 {
        v = increment(v, i.wrapping_sub(1), round, total);
        v = increment(v, i + SIZE - 1, round, total);
        v = increment(v, i.wrapping_sub(SIZE).wrapping_sub(1), round, total);
    }
    if (i % SIZE) != (SIZE - 1) {
        v = increment(v, i + 1, round, total);
        v = increment(v, i + SIZE + 1, round, total);
        v = increment(v, i.wrapping_sub(SIZE) + 1, round, total);
    }
    v = increment(v, i.wrapping_sub(SIZE), round, total);
    v = increment(v, i + SIZE, round, total);

    v
}

fn increment<'a>(mut v: &'a mut [State], i: usize, round: u8, total: &mut u16) -> &'a mut [State] {
    match v.get_mut(i) {
        Some(state) => match state {
            State::Energy(energy) => {
                if *energy == 9 {
                    *state = State::Flashed(round);
                    *total += 1;
                    v = flash_around(v, i, round, total);
                } else {
                    *energy += 1;
                }
            }

            State::Flashed(cell_round) if *cell_round != round => {
                *state = State::Energy(1);
            }

            _ => {}
        },

        None => {}
    };

    v
}

#[aoc(day11, part1)]
pub fn part1(input: &[State]) -> u16 {
    let mut v = input.to_vec();

    let mut total = 0;
    for round in 0..100 {
        for i in 0..v.len() {
            increment(&mut v, i, round, &mut total);
        }
    }

    total
}

#[aoc(day11, part2)]
pub fn part2(input: &[State]) -> u16 {
    let mut v = input.to_vec();

    let mut step = 0;
    loop {
        step += 1;

        for i in 0..v.len() {
            increment(&mut v, i, step as u8, &mut 0);
        }

        let mut synced = true;
        for state in &mut v {
            match state {
                State::Energy(_) => {
                    synced = false;
                    break;
                }

                _ => {}
            }
        }

        if synced {
            return step;
        }
    }
}
