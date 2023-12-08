use std::cmp::Ordering;

use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("07");

    let mut hands = Vec::new();
    let mut jokers = Vec::new();

    for line in input {
        if let Ok(line) = line {
            let mut split = line.split(" ");
            let hand = split.next().unwrap();
            let bet = split.next().unwrap().parse::<u32>().unwrap();
            let mut chars = hand.chars();

            let mut nums: [[u8; 5]; 2] = [[0; 5]; 2];
            let mut hand_types = [0; 2];
            let mut freqs: [[u8; 15]; 2] = [[0; 15]; 2];

            for i in 0..5 {
                nums[0][i] = match chars.next().unwrap() {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    c => c as u8 - '0' as u8,
                };

                freqs[0][nums[0][i] as usize] += 1;
            }

            chars = hand.chars();
            
            let mut num_jokers = 0;

            for i in 0..5 {
                let mut joker = false;
                nums[1][i] = match chars.next().unwrap() {
                    'T' => 10,
                    'J' => {
                        num_jokers += 1;
                        joker = true;
                        1
                    }
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    c => c as u8 - '0' as u8,
                };

                if joker {
                    continue;
                }

                freqs[1][nums[1][i] as usize] += 1;
            }

            freqs[0].sort();
            freqs[1].sort();

            freqs[1][14] += num_jokers;

            for i in 0..2 {
                let freq = freqs[i];

                match freq[14] {
                    5 => hand_types[i] = 6,
                    4 => hand_types[i] = 5,
                    3 => {
                        if freq[13] == 2 {
                            hand_types[i] = 4;
                        } else {
                            hand_types[i] = 3;
                        }
                    }
                    2 => {
                        if freq[13] == 2 {
                            hand_types[i] = 2;
                        } else {
                            hand_types[i] = 1;
                        }
                    }
                    _ => {}
                }
            }

            hands.push((nums[0], hand_types[0], bet));
            jokers.push((nums[1], hand_types[1], bet));
        }
    }

    let sort = |a: &([u8; 5], u32, u32), b: &([u8; 5], u32, u32)| {
        if a.1 < b.1 {
            return Ordering::Less;
        } else if a.1 > b.1 {
            return Ordering::Greater;
        }

        for i in 0..5 {
            if a.0[i] < b.0[i] {
                return Ordering::Less;
            } else if a.0[i] > b.0[i] {
                return Ordering::Greater;
            }
        }

        return Ordering::Equal;
    };

    hands.sort_by(sort);
    jokers.sort_by(sort);

    let mut sums = (0, 0);
    for i in 0..hands.len() {
        sums.0 += (hands[i].2 as usize * (i + 1)) as u32;
        sums.1 += (jokers[i].2 as usize * (i + 1)) as u32;
    }

    return sums;
}
