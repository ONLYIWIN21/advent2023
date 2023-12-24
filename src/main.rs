#![recursion_limit = "1024"]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
    solve!(day01);
    solve!(day02);
    solve!(day03);
    solve!(day04);
    solve!(day05);
    solve!(day06);
    solve!(day07);
    solve!(day08);
    solve!(day09);
    solve!(day10);
    solve!(day11);
//    solve!(day12);
    solve!(day13);
//    solve!(day14);
    solve!(day15);
    solve!(day16);
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
            let solution = $day::solve();
            let elapsed = now.elapsed();
            println!("{}: {}ms", stringify!($day), elapsed.as_millis());
            println!("  Part 1: {}", solution.0);
            println!("  Part 2: {}", solution.1);
        }
    };
}
