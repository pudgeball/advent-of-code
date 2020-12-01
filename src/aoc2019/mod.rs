mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub(crate) fn run_day(day: usize) {
    match day {
        1 => day1::calculate(),
        2 => day2::calculate(),
        3 => day3::calculate(),
        4 => day4::calculate(),
        5 => day5::calculate(),
        _ => unimplemented!("These puzzles were not completed"),
    }
}
