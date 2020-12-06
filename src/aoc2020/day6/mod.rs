//!

use {itertools::Itertools, std::collections::HashSet};

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| group.replace("\n", "").chars().into_iter().unique().count())
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut all_answers = group
                .split("\n")
                .map(|party| party.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();

            let mut common_answers = all_answers.pop().unwrap();
            for answers in all_answers {
                common_answers = common_answers
                    .iter()
                    .filter(|it| answers.contains(*it))
                    .cloned()
                    .collect();
            }
            common_answers.len()
        })
        .sum()
}

pub(crate) fn run() {
    let input = crate::util::load_input("aoc2020/day6/input.txt");
    println!("The part 1 answer is: {}", part_1(&input));
    println!("The part 2 answer is: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            part_1(
                r#"abcx
abcy
abcz"#
            ),
            6
        );

        let sample_input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        assert_eq!(part_1(sample_input), 11);
        assert_eq!(part_2(sample_input), 6);
    }
}
