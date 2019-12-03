use std::fs;

fn do_work(input: String) -> String {
    let mut op_codes = input
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    op_codes[1] = 12;
    op_codes[2] = 2;

    let mut idx = 0;
    loop {
        let op_1 = op_codes[idx];

        match op_1 {
            1 | 2 => {
                let op_2 = op_codes[idx + 1] as usize;
                let op_3 = op_codes[idx + 2] as usize;
                let op_4 = op_codes[idx + 3] as usize;

                if op_1 == 1 {
                    op_codes[op_4] = op_codes[op_2] + op_codes[op_3]
                } else {
                    op_codes[op_4] = op_codes[op_2] * op_codes[op_3]
                }
            }
            99 => break,
            _ => panic!("Unknown op code"),
        }
        idx += 4;
    }

    let mut returned = "".to_string();
    for code in op_codes {
        if returned.is_empty() {
            returned = format!("{}", code);
        } else {
            returned = format!("{},{}", returned, code);
        }
    }
    returned
}

pub fn cacluate() {
    let filename =
        "/users/nickmcguire/projects/github/advent-of-code/src/twenty_nineteen/two/input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("{}", do_work(contents));
}

#[test]
fn test_examples() {
    assert_eq!(
        "3500,9,10,70,2,3,11,0,99,30,40,50",
        do_work("1,9,10,3,2,3,11,0,99,30,40,50".to_string())
    );

    assert_eq!("2,0,0,0,99", do_work("1,0,0,0,99".to_string()));

    assert_eq!("2,3,0,6,99", do_work("2,3,0,3,99".to_string()));

    assert_eq!("2,4,4,5,99,9801", do_work("2,4,4,5,99,0".to_string()));

    assert_eq!(
        "30,1,1,4,2,5,6,0,99",
        do_work("1,1,1,4,99,5,6,0,99".to_string())
    );
}
