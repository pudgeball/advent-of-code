use std::fs;

fn do_work(input: Vec<i64>) -> (i64, i64) {
    let mut total_fuel_part_one: i64 = 0;
    let mut total_fuel_part_two: i64 = 0;

    for module_mass in input {
        // Part One
        let mut added_fuel = calculate_required_fuel(module_mass);
        total_fuel_part_one += added_fuel;

        // Part Two
        while added_fuel > 0 {
            total_fuel_part_two += added_fuel;
            added_fuel = calculate_required_fuel(added_fuel);
        }
    }

    (total_fuel_part_one, total_fuel_part_two)
}

pub fn calculate() {
    let filename =
        "/users/nickmcguire/projects/github/advent-of-code/src/twenty_nineteen/one/input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let input = contents
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let (total_fuel_part_one, total_fuel_part_two) = do_work(input);

    println!(
        "Part One: The entire required fuel is {}",
        total_fuel_part_one
    );
    println!(
        "Part Two: The entire required fuel is {}",
        total_fuel_part_two
    );
}

fn calculate_required_fuel(module_mass: i64) -> i64 {
    (module_mass / 3) - 2
}

#[test]
fn test_examples() {
    assert_eq!(2, do_work(vec![12]).0);
    assert_eq!(2, do_work(vec![14]).0);
    assert_eq!(654, do_work(vec![1969]).0);
    assert_eq!(33_583, do_work(vec![100_756]).0);

    assert_eq!(2, do_work(vec![14]).1);
    assert_eq!(966, do_work(vec![1969]).1);
    assert_eq!(50_346, do_work(vec![100_756]).1);
}
