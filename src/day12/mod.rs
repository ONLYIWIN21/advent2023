use crate::get_input;

pub fn solve() -> (u32, u64) {
    let input = get_input!("12");

    let mut totals = (0, 0);
    for line in input {
        if let Ok(line) = line {
            println!("{}", line);
            let mut groups = line
                .split(" ")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let mut unknowns = Vec::new();
            let mut broken = Vec::new();
            let mut len = 0;
            for (x, c) in line.split(" ").next().unwrap().char_indices() {
                len = x;
                match c {
                    '?' => unknowns.push(x),
                    '#' => broken.push(x),
                    _ => {}
                }
            }

            let num_arrangements = arrangements(
                &groups,
                groups.iter().sum::<u32>() - broken.len() as u32,
                0,
                &mut broken,
                &unknowns,
                len,
            );

            if broken.contains(&len) {
                totals.1 += (num_arrangements as u64).pow(5);
            } else if !unknowns.contains(&len) {
                for i in 0..unknowns.len() {
                    unknowns[i] += 1
                }
                for i in 0..broken.len() {
                    broken[i] += 1
                }
                unknowns.push(0);
                let new_num_arrangements = arrangements(
                    &groups,
                    groups.iter().sum::<u32>() - broken.len() as u32,
                    0,
                    &mut broken,
                    &unknowns,
                    len + 1,
                );

                totals.1 += (new_num_arrangements as u64).pow(4) * num_arrangements as u64;
            } else {
                let u_len = unknowns.len();
                let b_len = broken.len();
                let g_len = groups.len();
                for i in 0..1 {
                    unknowns.push((len + 1) * (i + 1) + i);

                    for j in 0..u_len {
                        unknowns.push(unknowns[j] + 1 + i + (i + 1) * (len + 1));
                    }

                    for j in 0..b_len {
                        broken.push(broken[j] + 1 + i + (i + 1) * (len + 1));
                    }

                    for j in 0..g_len {
                        groups.push(groups[j]);
                    }
                }

                println!("{:?} {:?} {:?}", unknowns, broken, groups);

                let new_num_arrangements = arrangements(
                    &groups,
                    groups.iter().sum::<u32>() - broken.len() as u32,
                    0,
                    &mut broken,
                    &unknowns,
                    len * 5 + 4,
                );

                totals.1 += (new_num_arrangements as u64 - num_arrangements as u64).pow(5);
            }

            totals.0 += num_arrangements;
        }
    }

    return totals;
}

fn arrangements(
    groups: &Vec<u32>,
    times: u32,
    start: usize,
    broken: &mut Vec<usize>,
    unknowns: &Vec<usize>,
    len: usize,
) -> u32 {
    if times == 0 {
        return is_valid(groups, broken, len);
    }

    let mut total = 0;
    for i in start..unknowns.len() {
        broken.push(unknowns[i]);
        total += arrangements(groups, times - 1, i + 1, broken, unknowns, len);
        broken.pop();
    }

    return total;
}

fn is_valid(groups: &Vec<u32>, broken: &Vec<usize>, len: usize) -> u32 {
    let mut curr_len = 0;
    let mut curr_index = 0;
    for i in 0..=len {
        if broken.contains(&i) {
            if curr_index == groups.len() {
                return 0;
            }
            curr_len += 1;
        } else if curr_len != 0 {
            if curr_len != groups[curr_index] {
                return 0;
            }
            curr_len = 0;
            curr_index += 1;
        }
    }

    if curr_len != 0 && curr_len != groups[(curr_index + 1).min(groups.len() - 1)] {
        return 0;
    }

    return 1;
}
