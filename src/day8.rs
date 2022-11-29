use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)> {
    input
        .lines()
        .map(|line| {
            let split = line.split(" | ");
            let mut parts = split.map(|part| {
                part.split_whitespace()
                    .map(|part| part.bytes().map(|c| c - 97).collect())
                    .collect()
            });
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect()
}

fn keep(all: &mut Vec<u8>, only: &[u8]) {
    let v = all.iter().copied().filter(|n| only.contains(n)).collect();
    *all = v;
}

#[aoc(day8, part1)]
pub fn part1(input: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> u16 {
    let mut total = 0;

    for (_all_ten, four) in input {
        for digit in four {
            match digit.len() {
                2 | 3 | 4 | 7 => {
                    total += 1;
                }

                _ => {}
            }
        }
    }

    total
}

macro_rules! check_number {
    ($input:ident $($n:ident) *) => {
        $input.len() == check_number!(@count $($n) *) && $($input.contains(&($n as u8)))&&*
    };

    (@count) => {
        0
    };

    (@count $a:ident $($rest:ident) *) => {
        check_number!(@count $($rest) *) + 1
    };
}

#[aoc(day8, part2)]
pub fn part2(input: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> u32 {
    let mut total = 0;

    for (all_ten, four) in input {
        let mut possible_segments = [
            (0..6).collect(),
            (0..6).collect(),
            (0..6).collect(),
            (0..6).collect(),
            (0..6).collect(),
            (0..6).collect(),
            (0..6).collect(),
        ];

        for digit in all_ten {
            match digit.len() {
                2 => {
                    for segment in digit {
                        keep(&mut possible_segments[*segment as usize], &[2, 5]);
                    }
                }

                3 => {
                    for segment in digit {
                        keep(&mut possible_segments[*segment as usize], &[0, 2, 5]);
                    }
                }

                4 => {
                    for segment in digit {
                        keep(&mut possible_segments[*segment as usize], &[1, 2, 3, 5]);
                    }
                }

                _ => {}
            }
        }

        let mut a = 0;
        for (i, segment) in possible_segments.iter_mut().enumerate() {
            if segment == &mut [0, 2, 5] {
                *segment = vec![0];
                a = i;
            }
        }

        let mut segment_bd = vec![];
        for (i, segment) in possible_segments.iter_mut().enumerate() {
            if segment == &mut [1, 2, 3, 5] {
                *segment = vec![1, 3];
                segment_bd.push(i);
            }
        }

        let mut d = 0;
        let mut b = 0;
        for digit in all_ten {
            if digit.len() == 6 {
                if !digit.contains(&((segment_bd[0]) as u8)) {
                    possible_segments[segment_bd[0]] = vec![3];
                    d = segment_bd[0];
                    possible_segments[segment_bd[1]] = vec![5];
                    b = segment_bd[1];
                    break;
                } else if !digit.contains(&((segment_bd[1]) as u8)) {
                    d = segment_bd[1];
                    possible_segments[segment_bd[1]] = vec![3];
                    b = segment_bd[0];
                    possible_segments[segment_bd[0]] = vec![5];
                    break;
                }
            }
        }

        let mut f = 7;
        let mut c = 7;
        for digit in all_ten {
            if digit.len() == 5 {
                if digit.contains(&(b as u8)) {
                    for (i, segment) in possible_segments.iter_mut().enumerate() {
                        if segment == &mut [2, 5] {
                            if digit.contains(&(i as u8)) {
                                *segment = vec![5];
                                f = i;
                            } else {
                                *segment = vec![2];
                                c = i;
                            }

                            if c != 7 && f != 7 {
                                break;
                            }
                        }
                    }
                    break;
                }
            }
        }

        let mut counts = [0u8; 7];
        for digit in all_ten {
            if digit.len() == 5 {
                for (i, segment) in possible_segments.iter_mut().enumerate() {
                    if segment.len() != 1 {
                        if digit.contains(&(i as u8)) {
                            counts[i] += 1;
                        }
                    }
                }
            }
        }
        let e = counts.iter().copied().position(|n| n == 1).unwrap();
        let g = counts.iter().copied().position(|n| n == 3).unwrap();

        let mut num = 0;
        for digit in four {
            num *= 10;

            if check_number!(digit a b c e f g) {
                num += 0;
            } else if check_number!(digit c f) {
                num += 1;
            } else if check_number!(digit a c d e g) {
                num += 2;
            } else if check_number!(digit a c d f g) {
                num += 3;
            } else if check_number!(digit b c d f) {
                num += 4;
            } else if check_number!(digit a b d f g) {
                num += 5;
            } else if check_number!(digit a b d e f g) {
                num += 6;
            } else if check_number!(digit a c f) {
                num += 7;
            } else if check_number!(digit a b c d e f g) {
                num += 8;
            } else if check_number!(digit a b c d f g) {
                num += 9;
            } else {
                panic!();
            }
        }

        total += num;
    }

    total
}
