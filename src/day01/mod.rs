use crate::get_input;

pub fn solve(part1: bool) -> u32 {
    let input = get_input!("01");
    let mut sum = 0;
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
            let mut first = true;
            let mut last = 0;
            for (i, c) in line.char_indices() {
                match c {
                    '0'..='9' => {
                        if first {
                            sum += to_int(c) * 10;
                            last = to_int(c);
                            first = false;
                        } else {
                            last = to_int(c);
                        }
                    }
                    _ => (),
                }
                if part1 {
                    continue;
                }
                for (n, s) in numbers {
                    if s.len() + i <= line.len() && &line[i..i + s.len()] == s {
                        if first {
                            sum += n * 10;
                            last = n;
                            first = false;
                        } else {
                            last = n;
                        }
                    }
                }
            }
            sum += last;
        }
    }
    return sum;
}

fn to_int(c: char) -> u32 {
    return c as u32 - '0' as u32;
}
