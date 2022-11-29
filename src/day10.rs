use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::Peekable;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            fn recurse<I: Iterator<Item = u8>>(
                mut input: &mut Peekable<I>,
            ) -> (Option<u8>, &mut Peekable<I>) {
                let closing = match input.next() {
                    Some(b'(') => b')',
                    Some(b'[') => b']',
                    Some(b'{') => b'}',
                    Some(b'<') => b'>',

                    Some(b) => return (Some(b), input),
                    None => return (None, input),
                };

                if Some(closing) == input.peek().copied() {
                    input.next();
                    return (None, input);
                } else {
                    loop {
                        let (success, new_input) = recurse(input);
                        input = new_input;
                        if let Some(b) = success {
                            return (Some(b), input);
                        } else {
                            if let Some(next) = input.peek().copied() {
                                if next == b')' || next == b']' || next == b'}' || next == b'>' {
                                    return if next == closing {
                                        input.next();
                                        (None, input)
                                    } else {
                                        (Some(next), input)
                                    };
                                }
                            } else {
                                return (None, input);
                            }
                        }
                    }
                }
            }

            let (error_at, _) = recurse(&mut line.bytes().peekable());

            match error_at {
                Some(b')') => 3,
                Some(b']') => 57,
                Some(b'}') => 1197,
                Some(b'>') => 25137,

                _ => 0,
            }
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &[String]) -> u64 {
    let mut scores: Vec<_> = input
        .iter()
        .filter_map(|line| {
            #[derive(Debug, PartialEq)]
            enum Needs {
                End,
                Needs(u8, u64, Box<Needs>),
            }

            let mut needs = Box::new(Needs::End);
            for b in line.bytes() {
                match b {
                    b'(' => needs = Box::new(Needs::Needs(b')', 1, needs)),
                    b'[' => needs = Box::new(Needs::Needs(b']', 2, needs)),
                    b'{' => needs = Box::new(Needs::Needs(b'}', 3, needs)),
                    b'<' => needs = Box::new(Needs::Needs(b'>', 4, needs)),

                    closing => match *needs {
                        Needs::Needs(char, _, new_needs) if char == closing => {
                            needs = new_needs;
                        }
                        _ => return None,
                    },
                }
            }

            let mut score = 0;
            while let Needs::Needs(_, wrapped_score, new_needs) = *needs {
                needs = new_needs;
                score *= 5;
                score += wrapped_score;
            }

            Some(score)
        })
        .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
}
