use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> HashMap<String, Vec<String>> {
    let iter = input.lines().map(|line| {
        let mut parts = line.split("-");

        (
            parts.next().unwrap().to_owned(),
            parts.next().unwrap().to_owned(),
        )
    });

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for (first, second) in iter {
        map.entry(first.clone()).or_default().push(second.clone());
        map.entry(second).or_default().push(first);
    }

    map
}

#[aoc(day12, part1)]
pub fn part1(input: &HashMap<String, Vec<String>>) -> u32 {
    fn recurse(
        paths: &HashMap<String, Vec<String>>,
        mut visited: Vec<String>,
        current: String,
    ) -> u32 {
        if current.to_lowercase() == current && visited.contains(&current) {
            return 0;
        }

        if current == "end" {
            return 1;
        }

        let v = paths.get(&current).unwrap();
        visited.push(current);

        v.iter()
            .map(|possible| recurse(paths, visited.clone(), possible.clone()))
            .sum::<u32>()
    }

    recurse(input, vec![], "start".to_owned())
}

#[aoc(day12, part2)]
pub fn part2(input: &HashMap<String, Vec<String>>) -> u32 {
    fn recurse(
        paths: &HashMap<String, Vec<String>>,
        mut visited: HashMap<String, u8>,
        current: String,
    ) -> u32 {
        let v = paths.get(&current).unwrap();

        if current == "start" {
            return 0;
        }

        if current == "end" {
            return 1;
        }

        if current.to_lowercase() == current {
            let times_visited = visited.entry(current).or_default();
            *times_visited += 1;
            if *times_visited > 2 {
                return 0;
            }
            if *times_visited > 1 && visited.values().filter(|n| **n > 1).count() > 1 {
                return 0;
            }
        }

        v.iter()
            .map(|possible| recurse(paths, visited.clone(), possible.clone()))
            .sum()
    }

    input
        .get("start")
        .unwrap()
        .iter()
        .map(|possible| recurse(input, HashMap::new(), possible.clone()))
        .sum()
}
