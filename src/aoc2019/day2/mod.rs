use std::fs;

fn turn_vec_to_string(input: Vec<i64>) -> String {
    let mut returned = "".to_string();
    for code in input {
        if returned.is_empty() {
            returned = format!("{}", code);
        } else {
            returned = format!("{},{}", returned, code);
        }
    }
    returned
}

fn do_work(input: String) -> (i64, String) {
    let mut op_codes = input
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut idx = 0;
    loop {
        let (op_1, op_2, op_3, op_4) = (
            op_codes[idx],
            *op_codes.get(idx + 1).unwrap_or(&0) as usize,
            *op_codes.get(idx + 2).unwrap_or(&0) as usize,
            *op_codes.get(idx + 3).unwrap_or(&0) as usize,
        );

        match op_1 {
            1 => op_codes[op_4] = op_codes[op_2] + op_codes[op_3],
            2 => op_codes[op_4] = op_codes[op_2] * op_codes[op_3],
            99 => break,
            _ => panic!("Unknown op code"),
        }
        idx += 4;
    }

    let output = op_codes[0];
    let op_code_string = turn_vec_to_string(op_codes);
    (output, op_code_string)
}

pub fn calculate() {
    let filename =
        "/users/nickmcguire/projects/github/advent-of-code/src/twenty_nineteen/two/input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut op_codes = contents
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    op_codes[1] = 12;
    op_codes[2] = 2;

    let (output, op_codes_string) = do_work(turn_vec_to_string(op_codes.clone()));
    println!("{} => {}", output, op_codes_string);

    for noun in 0..99 {
        for verb in 0..99 {
            op_codes[1] = noun;
            op_codes[2] = verb;

            let (output, _) = do_work(turn_vec_to_string(op_codes.clone()));
            if output == 19_690_720 {
                println!("noun: {}, verb: {}", noun, verb);
            }
        }
    }
}

#[test]
fn test_examples() {
    use crate::aoc2019::day5::interpret_instructions;

    assert_eq!(
        "3500,9,10,70,2,3,11,0,99,30,40,50",
        interpret_instructions("1,9,10,3,2,3,11,0,99,30,40,50".to_string())
    );

    assert_eq!(
        "2,0,0,0,99",
        interpret_instructions("1,0,0,0,99".to_string())
    );

    assert_eq!(
        "2,3,0,6,99",
        interpret_instructions("2,3,0,3,99".to_string())
    );

    assert_eq!(
        "2,4,4,5,99,9801",
        interpret_instructions("2,4,4,5,99,0".to_string())
    );

    assert_eq!(
        "30,1,1,4,2,5,6,0,99",
        interpret_instructions("1,1,1,4,99,5,6,0,99".to_string())
    );

    assert_eq!(
        "4138687,12,2,2,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,24,1,19,10,28,2,13,23,140,1,5,27,141,2,6,31,282,1,6,35,284,2,39,9,852,1,5,43,853,1,13,47,858,1,10,51,862,2,55,10,3448,2,10,59,13792,1,9,63,13795,2,67,13,68975,1,71,6,68977,2,6,75,137954,1,5,79,137955,2,83,9,413865,1,6,87,413867,2,91,6,827734,1,95,6,827736,2,99,13,4138680,1,6,103,4138682,1,2,107,4138684,1,111,9,0,99,2,14,0,0",
        interpret_instructions("1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,19,10,23,2,13,23,27,1,5,27,31,2,6,31,35,1,6,35,39,2,39,9,43,1,5,43,47,1,13,47,51,1,10,51,55,2,55,10,59,2,10,59,63,1,9,63,67,2,67,13,71,1,71,6,75,2,6,75,79,1,5,79,83,2,83,9,87,1,6,87,91,2,91,6,95,1,95,6,99,2,99,13,103,1,6,103,107,1,2,107,111,1,111,9,0,99,2,14,0,0".to_string())
    );
}
