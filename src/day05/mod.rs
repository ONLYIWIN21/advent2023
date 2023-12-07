use crate::get_input;

pub fn solve() -> (u32, u32) {
    let mut input = get_input!("05");

    let mut new_seeds = input
        .next()
        .unwrap()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut old_seeds = vec![0; new_seeds.len()];

    let mut old_range = Vec::new();
    for i in (0..old_seeds.len()).step_by(2) {
        old_range.push((new_seeds[i], new_seeds[i] + new_seeds[i + 1]));
    }
    let mut mappings: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut curr_map = Vec::new();

    for line in input {
        if let Ok(line) = line {
            if line.is_empty() {
                mappings.push(curr_map);
                curr_map = Vec::new();
                continue;
            }

            if line.contains(":") {
                for i in 0..new_seeds.len() {
                    old_seeds[i] = new_seeds[i];
                }
                continue;
            }

            let mut nums = line.split(" ");
            let start_new = nums.next().unwrap().parse::<u64>().unwrap();
            let start_old = nums.next().unwrap().parse::<u64>().unwrap();
            let range = nums.next().unwrap().parse::<u64>().unwrap();

            for i in 0..old_seeds.len() {
                let seed = old_seeds[i];
                if seed >= start_old && seed < start_old + range {
                    let new_seed = seed - start_old + start_new;
                    new_seeds[i] = new_seed;
                }
            }

            curr_map.push((start_new, start_old, range));
        }
    }


    let mut min = 99999999999;
    for (start, end) in old_range {
        let mut new_range = vec![(start, end)];
        for map in &mappings {
            let mut final_range = Vec::new();
            for (start_new, start_old, range) in map {
                let mut med_range = Vec::new();
                while !new_range.is_empty() {
                    let (seed_start, seed_end) = new_range.pop().unwrap();

                    let front_range = (seed_start, seed_end.min(*start_old));
                    let mid_range = (seed_start.max(*start_old), seed_end.min(start_old + range));
                    let back_range = (seed_start.max(start_old + range), seed_end);

                    if front_range.0 < front_range.1 {
                        med_range.push(front_range);
                    }

                    if mid_range.0 < mid_range.1 {
                        final_range.push((
                            mid_range.0 + start_new - start_old,
                            mid_range.1 + start_new - start_old,
                        ));
                    }

                    if back_range.0 < back_range.1 {
                        med_range.push(back_range);
                    }
                }
                new_range = med_range;
            }
            new_range.append(&mut final_range);
        }
        let this_min = new_range.iter().min().unwrap().0;
        if this_min < min {
            min = this_min;
        }
    }

    return (*new_seeds.iter().min().unwrap() as u32, min as u32);
}
