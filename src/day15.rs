use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<u8> {
    input
        .bytes()
        .filter_map(|b| b.checked_sub(48))
        .collect()
}

fn dijkstra(input: &[u8], size: usize) -> u32 {
    let mut unvisited: Vec<_> = (0..(size * size)).collect();
    let mut tentative = vec![u32::MAX; size * size];
    tentative[0] = 0;

    let mut current_index = 0;
    let mut current = 0;
    loop {
        unvisited.swap_remove(current_index);

        let mut neighbors = vec![];
        if (current % size) != (size - 1) {
            neighbors.push(current + 1);
        }
        if (current + size) < (size * size) {
            neighbors.push(current + size);
        }
        if (current % size) != 0 {
            neighbors.push(current - 1);
        }
        if current >= size {
            neighbors.push(current - size);
        }

        for neighbor in neighbors {
            let dist = (input[neighbor] as u32) + tentative[current];

            if dist < tentative[neighbor] {
                tentative[neighbor] = dist;
            }
        }

        let new = unvisited
            .iter()
            .copied()
            .enumerate()
            .min_by_key(|(_, point)| tentative[*point])
            .unwrap();
        current_index = new.0;
        current = new.1;

        if current == (size * size - 1) {
            return tentative[current];
        }
    }
}

const SIZE: usize = 100;

#[aoc(day15, part1)]
pub fn part1(input: &[u8]) -> u32 {
    dijkstra(input, SIZE)
}

#[aoc(day15, part2)]
pub fn part2(input: &[u8]) -> u32 {
    let mut v: Vec<_> = input.chunks(SIZE).flat_map(|chunk| {
        let mut v = chunk.to_vec();
        v.extend(chunk.iter().copied().map(|x| x % 9 + 1));
        v.extend(chunk.iter().copied().map(|x| (x + 1) % 9 + 1));
        v.extend(chunk.iter().copied().map(|x| (x + 2) % 9 + 1));
        v.extend(chunk.iter().copied().map(|x| (x + 3) % 9 + 1));
        v
    }).collect();

    let v2 = v.clone();
    v.extend(v2.iter().copied().map(|x| x % 9 + 1));
    v.extend(v2.iter().copied().map(|x| (x + 1) % 9 + 1));
    v.extend(v2.iter().copied().map(|x| (x + 2) % 9 + 1));
    v.extend(v2.iter().copied().map(|x| (x + 3) % 9 + 1));

    dijkstra(&v, SIZE * 5)
}