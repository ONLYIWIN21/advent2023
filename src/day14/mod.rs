use std::collections::HashMap;

use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("14");

    let mut map = Vec::new();
    for line in input {
        if let Ok(line) = line {
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '#' => row.push(0),
                    'O' => row.push(1),
                    '.' => row.push(2),
                    _ => (),
                }
            }
            map.push(row);
        }
    }

    let mut prev = HashMap::new();

    let mut sums = (0, 0);
    let mut i = 0;
    while i < 1000000000 {
        for _ in 0..100 {
            for x in 0..map[0].len() {
                for y in (1..map.len()).rev() {
                    if map[y][x] == 1 && map[y - 1][x] == 2 {
                        map[y][x] = 2;
                        map[y - 1][x] = 1;
                    }
                }
            }
        }

        if sums.0 == 0 {
            for y in 0..map.len() {
                let mut row_count = 0;
                for x in 0..map[0].len() {
                    if map[y][x] == 1 {
                        row_count += 1;
                    }
                }
                sums.0 += row_count * (map.len() - y) as u32;
            }
        }

        for _ in 0..100 {
            for y in 0..map.len() {
                for x in (1..map[0].len()).rev() {
                    if map[y][x] == 1 && map[y][x - 1] == 2 {
                        map[y][x] = 2;
                        map[y][x - 1] = 1;
                    }
                }
            }
        }

        for _ in 0..100 {
            for x in 0..map[0].len() {
                for y in 0..(map.len() - 1) {
                    if map[y][x] == 1 && map[y + 1][x] == 2 {
                        map[y][x] = 2;
                        map[y + 1][x] = 1;
                    }
                }
            }
        }

        for _ in 0..100 {
            for y in 0..map.len() {
                for x in 0..(map[0].len() - 1) {
                    if map[y][x] == 1 && map[y][x + 1] == 2 {
                        map[y][x] = 2;
                        map[y][x + 1] = 1;
                    }
                }
            }
        }

        if prev.contains_key(&map) {
            i = prev[&map] + (1000000000 - prev[&map]) / (i - prev[&map]) * (i - prev[&map]);
        }

        prev.insert(map.clone(), i);
        i += 1;
    }

    for y in 0..map.len() {
        let mut row_count = 0;
        for x in 0..map[0].len() {
            if map[y][x] == 1 {
                row_count += 1;
            }
        }
        sums.1 += row_count * (map.len() - y) as u32;
    }

    return sums;
}
