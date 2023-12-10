use std::collections::HashMap;

use crate::get_input;

pub fn solve() -> (u32, u64) {
    let mut input = get_input!("08");
    let instructions = input
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| {
            return match c {
                'L' => false,
                'R' => true,
                _ => panic!("Invalid input"),
            };
        })
        .collect::<Vec<bool>>();
    input.next();
    let mut map = HashMap::new();

    let mut starts = Vec::new();

    for line in input {
        if let Ok(line) = line {
            let chars = line.chars().collect::<Vec<char>>();
            let mut key = 0;
            let mut values = (0, 0);
            for i in 0..3 {
                key *= 26;
                values.0 *= 26;
                values.1 *= 26;
                key += to_int(chars[i]);
                values.0 += to_int(chars[i + 7]);
                values.1 += to_int(chars[i + 12]);
            }
            if chars[2] == 'A' {
                starts.push(key);
            }
            map.insert(key, values);
        }
    }

    let mut steps1 = 0;
    let mut x = 0;
    while x != to_int('Z') * 703 {
        if instructions[steps1 % instructions.len()] {
            x = map[&x].1;
        } else {
            x = map[&x].0;
        }
        steps1 += 1;
    }

    let mut steps2 = Vec::new();
    for start in starts {
        let mut steps = 0;
        let mut x = start;
        while x % 26 != to_int('Z') {
            if instructions[steps % instructions.len()] {
                x = map[&x].1;
            } else {
                x = map[&x].0;
            }
            steps += 1;
        }
        steps2.push(steps);
    }

    let mut a = steps2[0];
    let mut b;
    for i in 1..steps2.len() {
        b = steps2[i].min(a);
        a = steps2[i].max(a);

        let mut prev = a;
        let mut rem = b;
        
        while rem != 0 {
            let temp = rem;
            rem = prev % rem;
            prev = temp;
        }

        a = a / prev * b;
    }

    return (steps1 as u32, a as u64);
}

fn to_int(c: char) -> u32 {
    return c as u32 - 'A' as u32;
}
