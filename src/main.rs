mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    solve!(day01);
    solve!(day02);
    solve!(day03);
    solve!(day04);
}

#[macro_export]
macro_rules! get_input {
    ($day:literal) => {
        {
            use std::fs::File;
            use std::io::{self, BufRead};
            let file = File::open(concat!("src/day", $day, "/input")).unwrap();
            io::BufReader::new(file).lines()
        }
    };
}

#[macro_export]
macro_rules! solve {
    ($day:ident) => {
        {
            use std::time::Instant;
            let now = Instant::now();
            let part1 = $day::solve(true);
            let part2 = $day::solve(false);
            let elapsed = now.elapsed();
            println!("{}: {}ms", stringify!($day), elapsed.as_millis());
            println!("  Part 1: {}", part1);
            println!("  Part 2: {}", part2);
        }
    };
}
