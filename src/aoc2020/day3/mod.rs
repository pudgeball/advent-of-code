//!

fn traverse_map(input: &str, column_stride: usize, row_stride: usize) -> i64 {
    let mut column = column_stride;
    let mut row = row_stride;

    let puzzle_width = input.lines().next().unwrap().len();
    let mut hit_count = 0;

    while row < input.lines().count() {
        let column_position = if column >= puzzle_width {
            column - (puzzle_width * (column / puzzle_width))
        } else {
            column
        };

        let map_char = input.lines().nth(row).unwrap().chars().nth(column_position);

        if map_char == Some('#') {
            hit_count += 1;
        }

        column += column_stride;
        row += row_stride;
    }

    hit_count
}

fn part_1(input: &str) -> i64 {
    traverse_map(input, 3, 1)
}

fn part_2(input: &str) -> i64 {
    traverse_map(input, 1, 1)
        * traverse_map(input, 3, 1)
        * traverse_map(input, 5, 1)
        * traverse_map(input, 7, 1)
        * traverse_map(input, 1, 2)
}

pub(crate) fn run() {
    let input = crate::util::load_input("aoc2020/day3/input.txt");
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
                r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#
            ),
            7
        );

        assert_eq!(
            part_2(
                r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#
            ),
            336
        );
    }
}
