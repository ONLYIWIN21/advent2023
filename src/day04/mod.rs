use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("04");

    let mut sum = 0;
    let mut scratch_cards: [u64; 213] = [1; 213];
    let mut card_num = 1;
    for line in input {
        if let Ok(line) = line {
            let mut score = 1;
            let mut split = line.split("|");
            let solns: Vec<u32> = split
                .next()
                .unwrap()
                .split_whitespace()
                .skip(2)
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            let mut num_wins = 0;
            for guess in split.next().unwrap().split_whitespace() {
                let guess = guess.parse::<u32>().unwrap();
                if solns.contains(&guess) {
                    score *= 2;
                    num_wins += 1;
                }
            }

            for i in (card_num + 1)..=(num_wins + card_num) {
                scratch_cards[i] += scratch_cards[card_num];
            }

            sum += score / 2;
        }
        card_num += 1;
    }

    let mut total = 0;
    for card in scratch_cards {
        total += card;
    }

    return (sum, (total - 1) as u32);
}
