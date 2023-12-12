use crate::get_input;

pub fn solve() -> (usize, usize) {
    let input = get_input!("11");

    let mut map = Vec::new();
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    let mut cols = Vec::new();
    let mut y: usize = 0;
    for line in input {
        if let Ok(line) = line {
            let mut found = false;
            for (x, c) in line.char_indices() {
                if y == 0 {
                    cols.push(0);
                }
                if c == '#' {
                    map.push((x, y));
                    cols[x] += 1;
                    found = true;
                }
            }
            if !found {
                empty_rows.push(y);
            }
            y += 1;
        }
    }

    for (x, col) in cols.iter().enumerate() {
        if *col == 0 {
            empty_cols.push(x);
        }
    }

    let mut sums = (0, 0);
    for i in 0..map.len() {
        for j in (i + 1)..map.len() {
            let len = map[i].0.abs_diff(map[j].0) + map[i].1.abs_diff(map[j].1);
            let mut curr_sums = (len, len);
            for row in empty_rows.iter() {
                if map[i].1.min(map[j].1) < *row && map[j].1.max(map[i].1) > *row {
                    curr_sums.0 += 1;
                    curr_sums.1 += 999999;
                }
            }

            for col in empty_cols.iter() {
                if map[i].0.min(map[j].0) < *col && map[j].0.max(map[i].0) > *col {
                    curr_sums.0 += 1;
                    curr_sums.1 += 999999;
                }
            }

            sums.0 += curr_sums.0;
            sums.1 += curr_sums.1;
        }
    }

    return sums;
}
