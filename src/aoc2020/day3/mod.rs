//!

fn part_1(input: &str) -> usize {
    let mut column = 3;
    let mut row = 1;

    let puzzle_width = input.lines().next().unwrap().len();

    /*
    ..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#

        **/
    /*
    ..##.........##.........##.........##.........##.........##.......  --->
    #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
    .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
    ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
    .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
    ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
    .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
    .#........#.#........X.#........#.#........#.#........#.#........#
    #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
    #...##....##...##....##...#X....##...##....##...##....##...##....#
    .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

    */
    let mut hit_count = 0;

    while row < input.lines().count() {
        let column_position = if column > puzzle_width {
            column - (puzzle_width * (column / puzzle_width))
        } else {
            column
        };

        let map_char = input.lines().nth(row).unwrap().chars().nth(column_position);

        if map_char == Some('#') {
            hit_count += 1;
        }

        // println!(
        //     "Row<{}>, Column<{}>: char<{:?}> from position {}",
        //     row, column, map_char, column_position
        // );

        column += 3;
        row += 1;
    }

    hit_count
}

fn part_2(input: &str) -> usize {
    0
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
        )
    }
}
