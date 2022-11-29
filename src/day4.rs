use crate::opt_parse;
use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input {
    numbers: Vec<u16>,
    boards: Vec<Vec<Vec<u16>>>,
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Input {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let numbers = opt_parse::parse_u16_lines(first); // works because comma < 48

    lines.next();

    let mut boards = Vec::with_capacity(100);
    for _ in 0..100 {
        let mut rows = Vec::with_capacity(5);

        for _ in 0..5 {
            let line = lines.next().unwrap();
            let nums = line
                .split_ascii_whitespace()
                .map(|p| p.parse().unwrap())
                .collect();

            rows.push(nums);
        }

        boards.push(rows);

        lines.next();
    }

    Input { numbers, boards }
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> u16 {
    let mut win_conds = Vec::with_capacity(100 * (5 + 5));

    for (index, board) in input.boards.iter().enumerate() {
        let mut conds = Vec::with_capacity(5 + 5);

        for i in 0..5 {
            let row = board.get(i).unwrap();

            conds.push([row[0], row[1], row[2], row[3], row[4]]);

            conds.push([
                board[0][i],
                board[1][i],
                board[2][i],
                board[3][i],
                board[4][i],
            ]);
        }

        win_conds.push((index, conds));
    }

    let mut called = Vec::with_capacity(8);

    for num in &input.numbers {
        called.push(*num);

        for (board, conds) in &win_conds {
            if conds
                .iter()
                .any(|cond| cond.iter().all(|n| called.contains(n)))
            {
                return input
                    .boards
                    .get(*board)
                    .unwrap()
                    .iter()
                    .flatten()
                    .filter(|n| !called.contains(*n))
                    .sum::<u16>()
                    * num;
            }
        }
    }

    panic!("Could not find answer");
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> u16 {
    let mut win_conds = Vec::with_capacity(100 * (5 + 5));

    for (index, board) in input.boards.iter().enumerate() {
        let mut conds = Vec::with_capacity(5 + 5);

        for i in 0..5 {
            let row = board.get(i).unwrap();

            conds.push([row[0], row[1], row[2], row[3], row[4]]);
            conds.push([
                board[0][i],
                board[1][i],
                board[2][i],
                board[3][i],
                board[4][i],
            ]);
        }

        win_conds.push((index, conds));
    }

    let mut called = Vec::with_capacity(64);

    for num in &input.numbers {
        called.push(*num);

        let mut to_remove = vec![];
        let mut index = 0;
        for (board, conds) in &win_conds {
            if conds
                .iter()
                .any(|cond| cond.iter().all(|n| called.contains(n)))
            {
                to_remove.push(index);
                if (win_conds.len() - to_remove.len()) == 0 {
                    return input
                        .boards
                        .get(*board)
                        .unwrap()
                        .iter()
                        .flatten()
                        .filter(|n| !called.contains(*n))
                        .sum::<u16>()
                        * num;
                }
            } else {
                index += 1;
            }
        }

        for removable in to_remove {
            win_conds.remove(removable);
        }
    }

    panic!("Could not find answer");
}
