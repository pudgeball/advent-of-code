#![warn(clippy::all)]

mod aoc2019;
mod aoc2020;

pub(crate) mod util;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let year = args
        .get(1)
        .expect("Please call `run` with `{year} {day}`")
        .parse::<usize>()
        .expect("Please provide year as a number");

    let day = args
        .get(2)
        .expect("Please call `run` with `{year} {day}`")
        .parse::<usize>()
        .expect("Please provide year as a number");

    println!("{:?}", args);

    match year {
        2019 => aoc2019::run_day(day),
        2020 => aoc2020::run_day(day),
        _ => unimplemented!("Year not implemented"),
    }
}
