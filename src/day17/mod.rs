use crate::get_input;
use std::collections::HashMap;

pub fn solve() -> (usize, usize) {
    let input = get_input!("17");

    let grid = input
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c as usize - '0' as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut hist = HashMap::new();
    let least = fewest(0, 0, 0, (0, 1), &grid, &mut hist, 0);

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if let Some(dist) = hist.get(&(x, y)) {
                print!("{:04}|", dist.1 % 10000);
            } else {
                print!("    |");
            }
        }
        println!();
    }

    return (least, 0);
}

fn fewest(
    x: usize,
    y: usize,
    in_a_row: usize,
    dir: (usize, usize),
    grid: &Vec<Vec<usize>>,
    hist: &mut HashMap<(usize, usize), ((usize, usize), usize)>,
    n: usize,
) -> usize {
    if n > 330 {
        return 9999999999;
    }

    if x == grid.len() - 1 && y == grid[0].len() - 1 {
        return grid[y][x];
    }

    let mut min = 9999999999;

    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        if -dx == dir.0 as i32 && -dy == dir.1 as i32 {
            continue;
        }

        if in_a_row == 3 && dx == dir.0 as i32 && dy == dir.1 as i32 {
            continue;
        }

        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x < 0 || new_y < 0 || new_x >= grid.len() as i32 || new_y >= grid[0].len() as i32 {
            continue;
        }

        if let Some((_, m)) = hist.get(&(new_x as usize, new_y as usize)) {
            min = min.min(grid[new_y as usize][new_x as usize] + *m);
            continue;
        }

        let mut in_a_row = in_a_row;
        if dx == dir.0 as i32 && dy == dir.1 as i32 {
            in_a_row += 1;
        } else {
            in_a_row = 0;
        }

        min = min.min(
            grid[new_y as usize][new_x as usize]
                + fewest(
                    new_x as usize,
                    new_y as usize,
                    in_a_row,
                    (dx as usize, dy as usize),
                    grid,
                    hist,
                    n + 1,
                ),
        );
    }

    if min > 10000 {
        return min;
    }

    hist.insert((x, y), (dir, min));

    return min;
}
