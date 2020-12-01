//!

fn find_two_value_answer(input: &str) -> usize {
    let values = crate::util::lines_to_vec::<usize>(input);

    for value in values.iter() {
        for second_value in values.iter() {
            if value + second_value == 2020 {
                return value * second_value;
            }
        }
    }
    0
}

fn find_three_value_answer(input: &str) -> usize {
    let values = crate::util::lines_to_vec::<usize>(input);

    for value in values.iter() {
        for second_value in values.iter() {
            for third_value in values.iter() {
                if value + second_value + third_value == 2020 {
                    return value * second_value * third_value;
                }
            }
        }
    }
    0
}

pub(crate) fn run() {
    let input = crate::util::load_input("aoc2020/day1/input.txt");
    println!("The part 1 answer is: {}", find_two_value_answer(&input));
    println!("The part 2 answer is: {}", find_three_value_answer(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            find_two_value_answer(
                r#"1721
979
366
299
675
1456"#
            ),
            514579
        );

        assert_eq!(
            find_three_value_answer(
                r#"1721
979
366
299
675
1456"#
            ),
            241861950
        );
    }
}
