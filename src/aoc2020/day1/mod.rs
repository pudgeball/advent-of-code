//!

fn find_answer(input: &str) -> usize {
    let values = input
        .lines()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for value in values.iter() {
        for second_value in values.iter() {
            if value + second_value == 2020 {
                return value * second_value;
            }
        }
    }
    0
}

pub(crate) fn run() {
    let filename = "/users/nickmcguire/projects/github/advent-of-code/src/aoc2020/day1/input.txt";

    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("The answer is: {}", find_answer(&contents));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day() {
        assert_eq!(
            find_answer(
                r#"1721
979
366
299
675
1456"#
            ),
            514579
        );
    }
}
