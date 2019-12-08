use std::fs;
use std::io;

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

#[derive(Clone, Debug, PartialEq)]
enum OpCode {
    /// Adds together numbers read from two positions and stores the result in a third position
    Add {
        p1: i64,
        p2: i64,
        p3: i64,
    },
    /// works exactly like opcode 1, except it multiplies the two inputs instead of adding them
    Multiply {
        p1: i64,
        p2: i64,
        p3: i64,
    },
    /// Takes a single integer as input and saves it to the position given by its only parameter
    Input {
        p1: i64,
    },
    /// Outputs the value of its only parameter
    Output {
        p1: i64,
    },
    /// If the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing
    JumpIfTrue {
        p1: i64,
        p2: i64,
    },
    /// If the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing
    JumpIfFalse {
        p1: i64,
        p2: i64,
    },
    /// If the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0
    LessThan {
        p1: i64,
        p2: i64,
        p3: i64,
    },
    /// If the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0
    Equals {
        p1: i64,
        p2: i64,
        p3: i64,
    },
    Quit,
}

fn parse_digits(n: i64) -> Vec<i64> {
    let mut digits: Vec<i64> = Vec::new();

    let mut remainder = n;
    while remainder > 0 {
        digits.push(remainder % 10);
        remainder /= 10;
    }

    digits
}

fn get_param_value(mode: i64, value: i64, values: &[i64]) -> i64 {
    if mode == 0 {
        values[value as usize]
    } else {
        value
    }
}

// Hack idea, the output field is always last

fn update_parameters(
    op_code: OpCode,
    p1_mode: i64,
    p2_mode: i64,
    _p3_mode: i64,
    values: &[i64],
) -> OpCode {
    match op_code {
        OpCode::Add { p1, p2, p3 } => OpCode::Add {
            p1: get_param_value(p1_mode, p1, values),
            p2: get_param_value(p2_mode, p2, values),
            p3,
        },
        OpCode::Multiply { p1, p2, p3 } => OpCode::Multiply {
            p1: get_param_value(p1_mode, p1, values),
            p2: get_param_value(p2_mode, p2, values),
            p3,
        },
        OpCode::JumpIfTrue { p1, p2 } => OpCode::JumpIfTrue {
            p1: get_param_value(p1_mode, p1, values),
            p2: get_param_value(p2_mode, p2, values),
        },
        OpCode::JumpIfFalse { p1, p2 } => OpCode::JumpIfFalse {
            p1: get_param_value(p1_mode, p1, values),
            p2: get_param_value(p2_mode, p2, values),
        },
        OpCode::LessThan { p1, p2, p3 } => OpCode::LessThan {
            p1: get_param_value(p1_mode, p1, values),
            p2: get_param_value(p2_mode, p2, values),
            p3,
        },
        OpCode::Equals { p1, p2, p3 } => OpCode::Equals {
            p1: get_param_value(p1_mode, p1, values),
            p2: get_param_value(p2_mode, p2, values),
            p3,
        },
        _ => op_code,
    }
}

pub fn interpret_instructions(input: String) -> String {
    let mut values = input
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut idx = 0;
    loop {
        let value = values[idx];

        let digits = parse_digits(value);

        let raw_op_code =
            digits.get(0).expect("First digit is needed") + digits.get(1).unwrap_or(&0) * 10;
        let (p1_mode, p2_mode, p3_mode) = (
            *digits.get(2).unwrap_or(&0),
            *digits.get(3).unwrap_or(&0),
            *digits.get(4).unwrap_or(&0),
        );

        let (p1, p2, p3) = (
            *values.get(idx + 1).unwrap_or(&0),
            *values.get(idx + 2).unwrap_or(&0),
            *values.get(idx + 3).unwrap_or(&0),
        );

        let instruction_op_code: OpCode = match raw_op_code {
            1 => OpCode::Add { p1, p2, p3 },
            2 => OpCode::Multiply { p1, p2, p3 },
            3 => OpCode::Input { p1 },
            4 => OpCode::Output { p1 },
            5 => OpCode::JumpIfTrue { p1, p2 },
            6 => OpCode::JumpIfFalse { p1, p2 },
            7 => OpCode::LessThan { p1, p2, p3 },
            8 => OpCode::Equals { p1, p2, p3 },
            99 => OpCode::Quit,
            _ => panic!("Bad op code {}", raw_op_code),
        };
        let instruction_op_code =
            update_parameters(instruction_op_code, p1_mode, p2_mode, p3_mode, &values);
        //    println!("idx({}): {:?} - {:?}", idx, instruction_op_code, values);

        match instruction_op_code {
            OpCode::Add { p1, p2, p3 } => {
                idx += 4;
                values[p3 as usize] = p1 + p2
            }
            OpCode::Multiply { p1, p2, p3 } => {
                idx += 4;
                values[p3 as usize] = p1 * p2
            }
            OpCode::Input { p1 } => {
                let mut buffer = String::new();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("Failed to read from stdin");
                let input = buffer.trim().parse::<i64>().expect("Unable to parse");

                idx += 2;
                values[p1 as usize] = input;
            }
            OpCode::Output { p1 } => {
                idx += 2;
                println!("{}", values[p1 as usize])
            }
            OpCode::JumpIfTrue { p1, p2 } => {
                if p1 > 0 {
                    idx = p2 as usize;
                } else {
                    idx += 3;
                }
            }
            OpCode::JumpIfFalse { p1, p2 } => {
                if p1 == 0 {
                    idx = p2 as usize;
                } else {
                    idx += 3;
                }
            }
            OpCode::LessThan { p1, p2, p3 } => {
                idx += 4;
                let is_less = if p1 < p2 { 1 } else { 0 };
                values[p3 as usize] = is_less;
            }
            OpCode::Equals { p1, p2, p3 } => {
                idx += 4;
                let equals = if p1 == p2 { 1 } else { 0 };
                values[p3 as usize] = equals;
            }
            OpCode::Quit => break,
        }
    }

    turn_vec_to_string(values)
}

pub fn calculate() {
    let filename = "/users/nickmcguire/projects/github/advent-of-code/src/aoc2019/day5/input.txt";
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("input: {}", &input);
    println!("output: {}", interpret_instructions(input));
}

#[test]
fn test_example() {
    assert_eq!(
        "3500,9,10,70,2,3,11,0,99,30,40,50",
        interpret_instructions("1,9,10,3,2,3,11,0,99,30,40,50".to_string())
    );
}
