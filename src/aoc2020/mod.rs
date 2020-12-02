//!

mod day1;
mod day2;

pub(crate) fn run_day(day: usize) {
    match day {
        1 => day1::run(),
        2 => day2::run(),
        _ => unimplemented!("These puzzles were not completed"),
    }
}
