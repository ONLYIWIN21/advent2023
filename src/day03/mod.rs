use crate::get_input;

pub fn solve(part1: bool) -> u32 {
    const INPUT_SIZE: usize = 140 + 1;
    const PERIOD: u8 = 10;
    const SYMBOL: u8 = 11;
    let input = get_input!("03");
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut parts: [[u8; INPUT_SIZE + 1]; INPUT_SIZE + 1] = [[0; INPUT_SIZE + 1]; INPUT_SIZE + 1];
    let mut gears: [[(u8, u32); INPUT_SIZE + 1]; INPUT_SIZE + 1] = [[(0, 1); INPUT_SIZE + 1]; INPUT_SIZE + 1];

    let mut y = 1;
    for line in input {
        if let Ok(line) = line {
            for (mut x, c) in line.char_indices() {
                x += 1;
                match c {
                    '0'..='9' => parts[y][x] = c as u8 - '0' as u8,
                    '.' => parts[y][x] = PERIOD,
                    '*' => {
                        parts[y][x] = SYMBOL;
                        gears[y][x] = (1, 1);
                    }
                    _ => parts[y][x] = SYMBOL,
                }
            }
        }
        y += 1;
    }

    for i in 0..parts.len() {
        parts[0][i] = PERIOD;
        parts[i][0] = PERIOD;
        parts[INPUT_SIZE][i] = PERIOD;
        parts[i][INPUT_SIZE] = PERIOD;
    }

    for y in 1..INPUT_SIZE {
        let mut x = 1;
        while x < INPUT_SIZE {
            if parts[y][x] == PERIOD || parts[y][x] == SYMBOL {
                x += 1;
                continue;
            }

            let mut gears_adj = Vec::new();
            let mut num = 0;
            let mut found = parts[y - 1][x - 1] == SYMBOL
                || parts[y][x - 1] == SYMBOL
                || parts[y + 1][x - 1] == SYMBOL;
            if gears[y - 1][x - 1].0 > 0 {
                gears[y - 1][x - 1].0 += 1;
                gears_adj.push((y - 1, x - 1));
            }
            if gears[y][x - 1].0 > 0 {
                gears[y][x - 1].0 += 1;
                gears_adj.push((y, x - 1));
            }
            if gears[y + 1][x - 1].0 > 0 {
                gears[y + 1][x - 1].0 += 1;
                gears_adj.push((y + 1, x - 1));
            }
            for i in x..INPUT_SIZE {
                if parts[y][i] != PERIOD && parts[y][i] != SYMBOL {
                    found |= parts[y + 1][i] == SYMBOL;
                    found |= parts[y - 1][i] == SYMBOL;
                    if gears[y - 1][i].0 > 0 {
                        gears[y - 1][i].0 += 1;
                        gears_adj.push((y - 1, i));
                    }
                    if gears[y + 1][i].0 > 0 {
                        gears[y + 1][i].0 += 1;
                        gears_adj.push((y + 1, i));
                    }
                    num *= 10;
                    num += parts[y][i] as u32;
                } else {
                    found |= parts[y - 1][i] == SYMBOL;
                    found |= parts[y][i] == SYMBOL;
                    found |= parts[y + 1][i] == SYMBOL;
                    if gears[y][i].0 > 0 {
                        gears[y][i].0 += 1;
                        gears_adj.push((y, i));
                    }
                    if gears[y - 1][i].0 > 0 {
                        gears[y - 1][i].0 += 1;
                        gears_adj.push((y - 1, i));
                    }
                    if gears[y + 1][i].0 > 0 {
                        gears[y + 1][i].0 += 1;
                        gears_adj.push((y + 1, i));
                    }
                    x += i - x;
                    break;
                }
            }

            if found {
                sum1 += num;
            }

            for i in 0..gears_adj.len() {
                let (gear_y, gear_x) = gears_adj[i];
                if gears[gear_y][gear_x].0 > 3 {
                    continue;
                }
                gears[gear_y][gear_x].1 *= num;
            }

            x += 1;
        }
    }

    if part1 {
        return sum1;
    }

    for y in 1..INPUT_SIZE {
        for x in 1..INPUT_SIZE {
            if gears[y][x].0 == 3 {
                sum2 += gears[y][x].1;
            }
        }
    }

    return sum2;
}
