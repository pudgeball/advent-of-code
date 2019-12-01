use std::fs;

pub fn cacluate() {
    let filename =
        "/users/nickmcguire/projects/github/advent-of-code/src/twenty_nineteen/one/input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut total_fuel: i64 = 0;

    for line in contents.lines() {
        let module_mass = line.parse::<i64>().unwrap();
        let added_fuel = (module_mass / 3) - 2;
        total_fuel = total_fuel + added_fuel;
    }
    println!("The entire required fuel is {}", total_fuel);
}
