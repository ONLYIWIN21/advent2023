use crate::get_input;

pub fn solve() -> (i32, i32) {
    let input = get_input!("09");

    let mut line_nums = Vec::new();
    for line in input {
        if let Ok(line) = line {
            line_nums.push(
                line.split(" ")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
    }

    let mut sums = (0, 0);
    for nums in line_nums {
        let mut diffs = nums;
        let mut len = diffs.len();
        let mut sum = 0;
        let mut mul = 1;
        while len > 0 {
            sum += mul * diffs[0];
            mul *= -1;
            for i in 0..(len - 1) {
                diffs[i] = diffs[i + 1] - diffs[i];
            }
            len -= 1;
        }

        sums.0 += diffs.into_iter().sum::<i32>();
        sums.1 += sum;
    }

    return sums;
}
