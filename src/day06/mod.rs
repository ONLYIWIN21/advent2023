use crate::get_input;

pub fn solve(part1: bool) -> u32 {
    let mut input = get_input!("06");

    let time_str = input.next().unwrap().unwrap();
    let distance_str = input.next().unwrap().unwrap();

    let times = time_str
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let distances = distance_str
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut mul = 1;
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let desc = time * time - 4 * distance;
        let x_min = (time - ((desc as f32).sqrt() + 1.0) as u32) / 2;
        let x_max = (time + (desc as f32).sqrt() as u32) / 2;

        mul *= x_max - x_min;
    }
    if part1 {
        return mul;
    }

    let time = time_str
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let dist = distance_str
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let desc = time * time - 4 * dist;
    let x_min = (time - ((desc as f64).sqrt() + 1.0) as u64) / 2;
    let x_max = (time + (desc as f64).sqrt() as u64) / 2;

    return (x_max - x_min) as u32;
}
