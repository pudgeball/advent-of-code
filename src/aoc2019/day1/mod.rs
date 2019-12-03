use std::fs;

pub fn calculate() {
    let filename =
        "/users/nickmcguire/projects/github/advent-of-code/src/twenty_nineteen/one/input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut total_fuel_part_one: i64 = 0;
    let mut total_fuel_part_two: i64 = 0;

    for line in contents.lines() {
        let module_mass = line.parse::<i64>().unwrap();
        // Part One
        let mut added_fuel = calculate_required_fuel(module_mass);
        total_fuel_part_one = total_fuel_part_one + added_fuel;

        // Part Two
        while added_fuel > 0 {
            total_fuel_part_two += added_fuel;
            added_fuel = calculate_required_fuel(added_fuel);
        }
    }
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
