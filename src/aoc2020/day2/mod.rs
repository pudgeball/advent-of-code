//!

#[derive(Debug)]
struct Password<'a> {
    required_character: char,
    minimum_character: usize,
    maximum_character: usize,
    password: &'a str,
}

impl<'a> Password<'a> {
    fn is_valid(&self) -> bool {
        let character_count = self
            .password
            .chars()
            .filter(|c| c == &self.required_character)
            .count();

        character_count >= self.minimum_character && character_count <= self.maximum_character
    }
}

impl<'a> From<&'a str> for Password<'a> {
    fn from(input: &'a str) -> Self {
        let pieces = input.split(" ").collect::<Vec<_>>();

        let (minimum_character, maximum_character) = {
            let pieces = pieces[0].split("-").collect::<Vec<_>>();

            (
                pieces[0].parse::<usize>().unwrap(),
                pieces[1].parse::<usize>().unwrap(),
            )
        };
        let required_character = pieces[1].chars().next().unwrap();

        Password {
            required_character,
            minimum_character,
            maximum_character,
            password: pieces[2],
        }
    }
}

fn part_1(input: &str) -> usize {
    let passwords = input.lines().map(|l| l.into()).collect::<Vec<Password>>();

    println!(
        "{:?}",
        passwords
            .iter()
            .filter(|p| !p.is_valid())
            .collect::<Vec<_>>()
    );

    passwords.iter().filter(|p| p.is_valid()).count()
}

fn part_2(input: &str) -> usize {
    0
}

pub(crate) fn run() {
    let input = crate::util::load_input("aoc2020/day2/input.txt");
    println!("The part 1 answer is: {}", part_1(&input));
    println!("The part 2 answer is: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            part_1(
                r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#
            ),
            2
        );
    }
}
