use crate::get_input;

pub fn solve(part1: bool) -> i32 {
    let input = get_input!("02");
    let colors = [("red", 12), ("green", 13), ("blue", 14)];
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut i = 1;
    for line in input {
        if let Ok(line) = line {
            let mut valid = true;
            let mut maxes = [0, 0, 0];
            for round in line.split(": ").nth(1).unwrap().split("; ") {
                for pull in round.split(", ") {
                    let mut pull = pull.split(" ");
                    let num = pull.next().unwrap().parse::<i32>().unwrap();
                    let color = pull.next().unwrap();
                    let mut j = 0;
                    for (name, max) in colors.iter() {
                        if color == *name {
                            if num > *max {
                                valid = false;
                            }
                            maxes[j] = maxes[j].max(num);
                        }
                        j += 1;
                    }
                }
            }
            if valid {
                sum1 += i;
            }
            sum2 += maxes[0] * maxes[1] * maxes[2];
        }
        i += 1;
    }
    if part1 {
        return sum1;
    }
    return sum2;
}
