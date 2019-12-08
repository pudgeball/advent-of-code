use std::fs;
// use std::io::{self, Read};

// #[derive(Debug)]
// struct InstructionCode {
//     op_code: i64,
//     p1_mode: i64,
//     p2_mode: i64,
//     p3_mode: i64,
// }

// fn parse_digits(n: i64) -> Vec<i64> {
//     let mut digits: Vec<i64> = Vec::new();

//     let mut remainder = n;
//     while remainder > 0 {
//         digits.push(remainder % 10);
//         remainder /= 10;
//     }

//     digits
// }

// fn parse_instruction_code(input: i64) -> InstructionCode {
//     let digits = parse_digits(input);

//     InstructionCode {
//         op_code: digits.get(0).expect("First digit is needed") + digits.get(1).unwrap_or(&0) * 10,
//         p1_mode: *digits.get(2).unwrap_or(&0),
//         p2_mode: *digits.get(3).unwrap_or(&0),
//         p3_mode: *digits.get(4).unwrap_or(&0),
//     }
// }

// fn parse_param(param_mode: i64, idx: usize, memory: &[i64]) -> i64 {
//     let immediate_mode = *memory.get(idx).unwrap_or(&0);
//     println!("{}, {}, {:?}", &idx, &immediate_mode, &memory);
//     match param_mode {
//         0 => memory[immediate_mode as usize], // Position mode
//         1 => immediate_mode,                  // immediate mode
//         _ => panic!("Unable to handle param_mode"),
//     }
// }

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

// fn interpret_op_codes(input: String) -> (i64, String) {
//     let mut op_codes = input
//         .split(',')
//         .map(|n| n.parse::<i64>().unwrap())
//         .collect::<Vec<i64>>();

//     println!("{:?}", &op_codes);

//     let mut idx = 0;
//     loop {
//         let instruction_point = parse_instruction_code(op_codes[idx]);
//         let (p1, p2, p3) = (
//             parse_param(instruction_point.p1_mode, idx + 1, &op_codes),
//             parse_param(instruction_point.p2_mode, idx + 2, &op_codes),
//             parse_param(instruction_point.p3_mode, idx + 3, &op_codes),
//         );

//         dbg!(&instruction_point, p1, p2, p3);

//         match instruction_point.op_code {
//             1 => op_codes[p3 as usize] = p1 + p2, // ADD
//             2 => op_codes[p3 as usize] = p1 * p2, // MULTIPLY
//             3 => {
//                 let mut buffer = String::new();
//                 io::stdin()
//                     .read_to_string(&mut buffer)
//                     .expect("Unable to read stdin");
//                 let input = buffer.parse::<i64>().expect("Unable to parse");

//                 op_codes[p1 as usize] = input;
//             }
//             4 => println!("{}", op_codes[parse_param(1, idx + 1, &op_codes) as usize]), // OUTPUT
//             99 => break,                                                                // QUIT
//             _ => panic!("Unknown op code"),
//         }

//         idx += match instruction_point.op_code {
//             1 | 2 => 4,
//             3 | 4 => 2,
//             _ => panic!("Unknown op code"),
//         };

//         // let (op_1, op_2, op_3, op_4) = (
//         //     op_codes[idx],
//         //     *op_codes.get(idx + 1).unwrap_or(&0) as usize,
//         //     *op_codes.get(idx + 2).unwrap_or(&0) as usize,
//         //     *op_codes.get(idx + 3).unwrap_or(&0) as usize,
//         // );

//         // match op_1 {
//         //     1 => op_codes[op_4] = op_codes[op_2] + op_codes[op_3],
//         //     2 => op_codes[op_4] = op_codes[op_2] * op_codes[op_3],
//         //     3 => "",
//         //     4 => "",
//         //     99 => break,
//         //     _ => panic!("Unknown op code"),
//         // }
//     }

//     let output = op_codes[0];
//     let op_code_string = turn_vec_to_string(op_codes);
//     (output, op_code_string)
// }

// pub fn calculate() {
//     let filename = "/users/nickmcguire/projects/github/advent-of-code/src/aoc2019/day5/input.txt";

//     let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

//     let (output, op_code_string) = interpret_op_codes(input);
//     println!("{}", op_code_string);
// }

#[derive(Clone, Debug, PartialEq)]
enum OpCode {
    Add { p1: i64, p2: i64, p3: i64 },
    Multiply { p1: i64, p2: i64, p3: i64 },
    Quit,
}

#[derive(Debug)]
struct Instruction {
    op_code: OpCode,
    p1_mode: i64,
    p2_mode: i64,
    p3_mode: i64,
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

// fn parse_op_code(input: &i64) -> (i64, ) {
//     let digits = parse_digits(input);

//     InstructionCode {
//         op_code: ,
//         p1_mode: *digits.get(2).unwrap_or(&0),
//         p2_mode: *digits.get(3).unwrap_or(&0),
//         p3_mode: *digits.get(4).unwrap_or(&0),
//     }
// }

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

        let instruction_op_code: OpCode = match raw_op_code {
            1 => OpCode::Add {
                p1: values[idx + 1],
                p2: values[idx + 2],
                p3: values[idx + 3],
            },
            2 => OpCode::Multiply {
                p1: values[idx + 1],
                p2: values[idx + 2],
                p3: values[idx + 3],
            },
            99 => OpCode::Quit,
            _ => panic!("Bad op code {}", raw_op_code),
        };

        let instruction = Instruction {
            op_code: instruction_op_code,
            p1_mode,
            p2_mode,
            p3_mode,
        };
        println!("{:?}", instruction);

        match instruction.op_code {
            OpCode::Add { p1, p2, p3 } => {
                idx += 4;
                values[p3 as usize] = values[p1 as usize] + values[p2 as usize]
            }
            OpCode::Multiply { p1, p2, p3 } => {
                idx += 4;
                values[p3 as usize] = values[p1 as usize] * values[p2 as usize]
            }
            OpCode::Quit => break,
        }
    }

    // for instruction in instructions {
    //     match instruction.op_code {
    //         OpCode::Add { p1, p2, p3 } => {
    //             memory[p3 as usize] = memory[p1 as usize] + memory[p2 as usize]
    //         }
    //         OpCode::Multiply { p1, p2, p3 } => {
    //             memory[p3 as usize] = memory[p1 as usize] * memory[p2 as usize]
    //         }
    //         OpCode::Quit => break,
    //     }
    // }

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
