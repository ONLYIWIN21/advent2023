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

    for _ in 0..100 { // im lazy
        for x in 0..map[0].len() {
            for y in (1..map.len()).rev() {
                if map[y][x] == 1 && map[y - 1][x] == 2 {
                    map[y][x] = 2;
                    map[y - 1][x] = 1;
                }
            }
        }
    }

    let mut sum = 0;
    for y in 0..map.len() {
        let mut row_count = 0;
        for x in 0..map[0].len() {
            if map[y][x] == 1 {
                row_count += 1;
            }
        }
        sum += row_count * (map.len() - y) as u32;
    }

    return (sum, 0);
}
