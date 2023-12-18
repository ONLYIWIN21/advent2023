use crate::get_input;

pub fn solve() -> (usize, usize) {
    let line = get_input!("15").next().unwrap().unwrap();
    let start = line.split(',');
    let mut sum = 0;
    for s in start {
        let mut hash = 0;
        for c in s.chars() {
            hash += c as usize;
            hash *= 17;
            hash %= 256;
        }
        sum += hash;
    }
    return (sum, 0);
}
