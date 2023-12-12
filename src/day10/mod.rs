use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("10");

    let mut map: Vec<Vec<((u32, u32), (u32, u32))>> = Vec::new();
    let mut s = (0, 0);

    let mut y = 0;
    for line in input {
        if let Ok(line) = line {
            let mut row: Vec<((u32, u32), (u32, u32))> = Vec::new();
            for (x, c) in line.char_indices() {
                let x = x as u32;
                match c {
                    '|' => row.push(((x, y + 1), (x, (y as i32 - 1).max(0) as u32))),
                    '-' => row.push(((x + 1, y), ((x as i32 - 1).max(0) as u32, y))),
                    'F' => row.push(((x, y + 1), (x + 1, y))),
                    '7' => row.push(((x, y + 1), ((x as i32 - 1).max(0) as u32, y))),
                    'J' => row.push((
                        (x, (y as i32 - 1).max(0) as u32),
                        ((x as i32 - 1).max(0) as u32, y),
                    )),
                    'L' => row.push(((x, (y as i32 - 1).max(0) as u32), (x + 1, y))),
                    'S' => {
                        row.push(((u32::MAX, u32::MAX), (u32::MAX, u32::MAX)));
                        s = (x, y);
                    }
                    _ => row.push(((u32::MAX, u32::MAX), (u32::MAX, u32::MAX))),
                }
            }
            map.push(row);
            y += 1;
        }
    }

    let mut is_set = false;
    let mut heading = (0, 0);
    let mut start = (0, 0);
    for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let pos = map[(s.1 as i32 + y) as usize][(s.0 as i32 + x) as usize];
        if pos.0 == s || pos.1 == s {
            start = ((s.0 as i32 + x) as u32, (s.1 as i32 + y) as u32);
            if !is_set {
                heading = (-*x, -*y);
                is_set = true;
            }
        }
    }

    let mut is_loop = vec![vec![false; map[0].len()]; map.len()];
    let mut len = 1;
    let mut prev = s;
    let mut pos = start;
    while pos != s {
        is_loop[pos.1 as usize][pos.0 as usize] = true;
        len += 1;
        let next = map[pos.1 as usize][pos.0 as usize];
        if next.0 == prev {
            prev = pos;
            pos = next.1;
        } else {
            prev = pos;
            pos = next.0;
        }
    }

    let mut is_inside = vec![vec![false; map[0].len()]; map.len()];
    let mut prev = s;
    let mut pos = start;
    let mut last_diff = (prev.0 as i32 - pos.0 as i32, prev.1 as i32 - pos.1 as i32);
    while pos != s {
        if (heading.0 == 0 && last_diff.1 != 0) || (heading.1 == 0 && last_diff.0 != 0) {
            match last_diff {
                (0, -1) => heading = (-1, 0),
                (0, 1) => heading = (1, 0),
                (-1, 0) => heading = (0, 1),
                (1, 0) => heading = (0, -1),
                _ => (),
            }
        }
        let next = map[pos.1 as usize][pos.0 as usize];
        if next.0 == prev {
            prev = pos;
            pos = next.1;
        } else {
            prev = pos;
            pos = next.0;
        }
        if !is_loop[(pos.1 as i32 + heading.1) as usize][(pos.0 as i32 + heading.0) as usize] {
            is_inside[(pos.1 as i32 + heading.1) as usize][(pos.0 as i32 + heading.0) as usize] =
                true;
        }
        last_diff = (pos.0 as i32 - prev.0 as i32, pos.1 as i32 - prev.1 as i32);
    }

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if is_loop[y][x] {
//                print!("X");
            } else if is_inside[y][x] {
//                print!("O");
            } else {
//                print!(" ");
            }
        }
//        println!();
    }

    return (len / 2, 0);
}
