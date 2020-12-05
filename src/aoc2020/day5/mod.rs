//!

#[derive(Debug, Eq, PartialEq)]
struct BoardingPass {
    row: usize,
    column: usize,
    seat_id: usize,
}

impl From<&str> for BoardingPass {
    fn from(input: &str) -> Self {
        let row = calc_bound(&input[0..7], 0, 127);
        let column = calc_bound(&input[7..], 0, 7);

        BoardingPass {
            row,
            column,
            seat_id: row * 8 + column,
        }
    }
}

fn calc_bound(letters: &str, mut lower_bound: usize, mut upper_bound: usize) -> usize {
    let mut characters = letters.chars();
    let last_character = characters.next_back().unwrap();

    for letter in characters {
        if letter == 'F' || letter == 'L' {
            upper_bound -= ((upper_bound - lower_bound) / 2) + 1;
        } else {
            lower_bound += ((upper_bound - lower_bound) / 2) + 1;
        }
    }

    if last_character == 'F' || last_character == 'L' {
        lower_bound
    } else {
        upper_bound
    }
}

fn part_1(input: &str) -> usize {
    let mut boarding_passes = input
        .lines()
        .map(|l| l.into())
        .collect::<Vec<BoardingPass>>();

    boarding_passes.sort_by_key(|bp| bp.seat_id);
    boarding_passes.reverse();
    boarding_passes.iter().next().unwrap().seat_id
}

fn part_2(input: &str) -> usize {
    let mut boarding_passes = input
        .lines()
        .map(|l| l.into())
        .collect::<Vec<BoardingPass>>();

    boarding_passes.sort_by_key(|bp| bp.seat_id);

    let mut seat_number = None;

    for boarding_pass in boarding_passes {
        match seat_number {
            Some(number) => {
                if number + 1 != boarding_pass.seat_id {
                    return number + 1;
                } else {
                    seat_number = Some(boarding_pass.seat_id);
                }
            }
            None => seat_number = Some(boarding_pass.seat_id),
        }
    }
    0
}

pub(crate) fn run() {
    let input = crate::util::load_input("aoc2020/day5/input.txt");
    println!("The part 1 answer is: {}", part_1(&input));
    println!("The part 2 answer is: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        let bp: BoardingPass = "FBFBBFFRLR".into();
        assert_eq!(
            bp,
            BoardingPass {
                row: 44,
                column: 5,
                seat_id: 357
            }
        );
        let bp: BoardingPass = "BFFFBBFRRR".into();
        assert_eq!(
            bp,
            BoardingPass {
                row: 70,
                column: 7,
                seat_id: 567
            }
        );
        let bp: BoardingPass = "FFFBBBFRRR".into();
        assert_eq!(
            bp,
            BoardingPass {
                row: 14,
                column: 7,
                seat_id: 119
            }
        );
        let bp: BoardingPass = "BBFFBBFRLL".into();
        assert_eq!(
            bp,
            BoardingPass {
                row: 102,
                column: 4,
                seat_id: 820
            }
        );
    }
}
