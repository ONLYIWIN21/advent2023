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

    let mut start = (0, 0);
    for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let pos = map[(s.1 as i32 + y) as usize][(s.0 as i32 + x) as usize];
        if pos.0 == s || pos.1 == s {
            start = ((s.0 as i32 + x) as u32, (s.1 as i32 + y) as u32);
        }
    }

    let mut is_loop = vec![vec![false; map[0].len() * 2]; map.len() * 2];
    let mut num_loop = 0;
    let mut len = 1;
    let mut prev = s;
    let mut pos = start;
    while pos != s {
        is_loop[pos.1 as usize * 2][pos.0 as usize * 2] = true;
        is_loop[pos.1 as usize + prev.1 as usize][pos.0 as usize + prev.0 as usize] = true;
        num_loop += 1;
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
    is_loop[pos.1 as usize * 2][pos.0 as usize * 2] = true;
    is_loop[pos.1 as usize + prev.1 as usize][pos.0 as usize + prev.0 as usize] = true;
    num_loop += 1;

    let mut new_map = vec![vec![false; map[0].len() * 2 - 1]; map.len() * 2 - 1];
    fill(&mut new_map, &is_loop, 0, 0);

    let mut count = 0;
    for y in (0..new_map.len()).step_by(2) {
        for x in (0..new_map[y].len()).step_by(2) {
            if new_map[y][x] {
                count += 1;
            }
        }
    }

    return (
        len / 2,
        (map[0].len() * map.len() - count - num_loop) as u32,
    );
}

pub fn fill(map: &mut Vec<Vec<bool>>, walls: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    if map[y][x] || walls[y][x] {
        return false;
    }

    map[y][x] = true;
    if x > 0 {
        fill(map, walls, x - 1, y);
    }
    if x < map[0].len() - 1 {
        fill(map, walls, x + 1, y);
    }
    if y > 0 {
        fill(map, walls, x, y - 1);
    }
    if y < map.len() - 1 {
        fill(map, walls, x, y + 1);
    }

    return false;
}
