use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("01");
    let mut sums = (0, 0);
    let numbers = [
        (0, "zero"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ];
    for line in input {
        if let Ok(line) = line {
            let mut firsts = (true, true);
            let mut lasts = (0, 0);
            for (i, c) in line.char_indices() {
                match c {
                    '0'..='9' => {
                        if firsts.0 {
                            sums.0 += to_int(c) * 10;
                            lasts.0 = to_int(c);
                            firsts.0 = false;
                        } else {
                            lasts.0 = to_int(c);
                        }

                        if firsts.1 {
                            sums.1 += to_int(c) * 10;
                            lasts.1 = to_int(c);
                            firsts.1 = false;
                        } else {
                            lasts.1 = to_int(c);
                        }
                    }
                    _ => (),
                }

                for (n, s) in numbers {
                    if s.len() + i <= line.len() && &line[i..i + s.len()] == s {
                        if firsts.1 {
                            sums.1 += n * 10;
                            lasts.1 = n;
                            firsts.1 = false;
                        } else {
                            lasts.1 = n;
                        }
                    }
                }
            }
            sums.0 += lasts.0;
            sums.1 += lasts.1;
        }
    }
    return sums;
}

fn to_int(c: char) -> u32 {
    return c as u32 - '0' as u32;
}
