use crate::get_input;

pub fn solve() -> (usize, usize) {
    let input = get_input!("13");

    let mut patterns = Vec::new();
    let mut pattern = Vec::new();

    for line in input {
        if let Ok(line) = line {
            if line == "" {
                patterns.push(pattern);
                pattern = Vec::new();
                continue;
            }

            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c == '#');
            }

            pattern.push(row);
        }
    }

    patterns.push(pattern);

    let mut sums = [0, 0];
    for bad in 0..2 {
        for pattern in &patterns {
            let mut found = false;
            for i in 0..(pattern.len() - 1) {
                let mut num_bads = 0;
                let mut exists = false;
                let mut good = true;
                for i2 in 0..=i.min(pattern.len() - i - 2) {
                    if !good {
                        break;
                    }
                    for j in 0..pattern[0].len() {
                        if pattern[i + i2 + 1][j] != pattern[i - i2][j] && num_bads >= bad {
                            good = false;
                            break;
                        } else if pattern[i + i2 + 1][j] != pattern[i - i2][j] {
                            num_bads += 1;
                        } else {
                            exists = true;
                        }
                    }
                }
                if good && exists {
                    sums[bad] += (i + 1) * 100;
                    println!("{}", (i + 1) * 100);
                    found = true;
                    break;
                }
            }

            if found {
                continue;
            }

            for i in 0..(pattern[0].len() - 1) {
                let mut good = true;
                let mut num_bads = 0;
                for i2 in 0..=i.min(pattern[0].len() - i - 2) {
                    if !good {
                        break;
                    }
                    for j in 0..pattern.len() {
                        if pattern[j][i + i2 + 1] != pattern[j][i - i2] && num_bads >= bad {
                            good = false;
                            break;
                        } else if pattern[j][i + i2 + 1] != pattern[j][i - i2] {
                            num_bads += 1;
                        }
                    }
                }
                if good {
                    sums[bad] += i + 1;
                    println!("{}", i + 1);
                    break;
                }
            }
        }
    }

    return (sums[0], sums[1]);
}
