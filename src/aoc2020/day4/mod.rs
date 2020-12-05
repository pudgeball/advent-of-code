//!

use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum HeightType {
    CM,
    IN,
}

#[derive(Debug)]
struct Passport {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expiration_year: Option<usize>,
    height: Option<(usize, HeightType)>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn part_2_valid(&self) -> bool {
        self.is_valid()
            && self.birth_year_valid()
            && self.issue_year_valid()
            && self.expiration_year_valid()
            && self.height_valid()
            && self.hair_color_valid()
            && self.eye_color_valid()
            && self.passport_id_valid()
    }

    fn birth_year_valid(&self) -> bool {
        let birth_year = self.birth_year.unwrap_or_default();
        birth_year >= 1920 && birth_year <= 2002
    }

    fn issue_year_valid(&self) -> bool {
        let issue_year = self.issue_year.unwrap_or_default();
        issue_year >= 2010 && issue_year <= 2020
    }

    fn expiration_year_valid(&self) -> bool {
        let expiration_year = self.expiration_year.unwrap_or_default();
        expiration_year >= 2020 && expiration_year <= 2030
    }

    fn height_valid(&self) -> bool {
        if let Some((height, measurement)) = self.height {
            match measurement {
                HeightType::CM => {
                    if height >= 150 && height <= 193 {
                        return true;
                    }
                }
                HeightType::IN => {
                    if height >= 59 && height <= 76 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn hair_color_valid(&self) -> bool {
        if let Some(hair_color) = self.hair_color.clone() {
            let mut chars = hair_color.chars();

            if chars.next() == Some('#')
                && chars.all(|c| {
                    c == 'a'
                        || c == 'b'
                        || c == 'c'
                        || c == 'd'
                        || c == 'e'
                        || c == 'f'
                        || c == '0'
                        || c == '1'
                        || c == '2'
                        || c == '3'
                        || c == '4'
                        || c == '5'
                        || c == '6'
                        || c == '7'
                        || c == '8'
                        || c == '9'
                })
            {
                return true;
            }
        }
        false
    }

    fn eye_color_valid(&self) -> bool {
        let eye_color = self.eye_color.clone().unwrap_or_default();
        match eye_color.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }

    fn passport_id_valid(&self) -> bool {
        let passport_id = self.passport_id.clone().unwrap_or_default();
        passport_id.len() == 9 && passport_id.parse::<usize>().ok().is_some()
    }
}

impl From<String> for Passport {
    fn from(input: String) -> Self {
        let mut dict = HashMap::new();
        for piece in input.split(" ") {
            let (key, value) = {
                let temp = piece.split(":").collect::<Vec<_>>();
                (temp[0].to_string(), temp[1].to_string())
            };

            dict.insert(key, value);
        }

        Passport {
            birth_year: dict.get("byr").and_then(|s| s.parse::<usize>().ok()),
            issue_year: dict.get("iyr").and_then(|s| s.parse::<usize>().ok()),
            expiration_year: dict.get("eyr").and_then(|s| s.parse::<usize>().ok()),
            height: dict.get("hgt").and_then(|s| {
                if s.contains("cm") {
                    Some((s.replace("cm", "").parse().unwrap(), HeightType::CM))
                } else if s.contains("in") {
                    Some((s.replace("in", "").parse().unwrap(), HeightType::IN))
                } else {
                    None
                }
            }),
            hair_color: dict.get("hcl").map(|s| s.clone()),
            eye_color: dict.get("ecl").map(|s| s.clone()),
            passport_id: dict.get("pid").map(|s| s.clone()),
            country_id: dict.get("cid").map(|s| s.clone()),
        }
    }
}

fn parse_lines<'a>(input: &'a str) -> impl IntoIterator<Item = String> {
    let mut output = vec![];
    let lines = input.lines();

    let mut current_input = String::new();

    for line in lines {
        if line == "" {
            output.push(current_input);
            current_input = String::new();
        } else {
            current_input = format!(
                "{}{}{}",
                current_input,
                if current_input.len() > 0 { " " } else { "" },
                line
            );
        }
    }
    if current_input.len() > 0 {
        output.push(current_input);
    }

    output
}

fn part_1(input: &str) -> usize {
    let passwords = parse_lines(input)
        .into_iter()
        .map(|l| l.into())
        .collect::<Vec<Passport>>();
    passwords.iter().filter(|p| p.is_valid()).count()
}

fn part_2(input: &str) -> usize {
    let passwords = parse_lines(input)
        .into_iter()
        .map(|l| l.into())
        .collect::<Vec<Passport>>();
    passwords.iter().filter(|p| p.part_2_valid()).count()
}

pub(crate) fn run() {
    let input = crate::util::load_input("aoc2020/day4/input.txt");
    println!("The part 1 answer is: {}", part_1(&input));
    println!("The part 2 answer is: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            part_1(
                r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#
            ),
            2
        );

        assert_eq!(
            part_2(
                r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#
            ),
            0
        );

        assert_eq!(
            part_2(
                r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#
            ),
            4
        );
    }
}
