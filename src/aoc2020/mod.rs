//!

mod day1;
mod day2;
mod day3;
mod day4;

pub(crate) fn run_day(day: usize) {
    match day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        _ => unimplemented!("These puzzles were not completed"),
    }
}
