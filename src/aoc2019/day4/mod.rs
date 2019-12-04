pub fn password_is_valid(input: i64) -> bool {
    if input < 100_000 || input > 999_999 {
        return false;
    }
    let input = input.to_string();

    let chars: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut sorted_chars: Vec<u32> = chars.clone();
    sorted_chars.sort();

    if !chars.iter().eq(sorted_chars.iter()) {
        return false;
    }

    let mut last_number: Option<u32> = None;
    for n in chars {
        if last_number.is_none() || last_number != Some(n) {
            last_number = Some(n)
        } else {
            return true;
        }
    }

    false
}

pub fn calculate() {
    let mut password_count = 0;

    for i in 347_312..805_915 {
        if password_is_valid(i) {
            password_count += 1;
        }
    }

    println!("Number of passwords is {}", password_count);
}

#[test]
fn test_examples() {
    assert_eq!(true, password_is_valid(111_111));
    assert_eq!(false, password_is_valid(223_450));
    assert_eq!(false, password_is_valid(123_789));
}
