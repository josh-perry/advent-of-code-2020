use std::fs;
use regex::Regex;

pub fn part_1(filename: String) -> i32 {
    let contents = fs::read_to_string(filename).expect("Failed to read file!");
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut good_passports = 0;

    let all_passport_strings : Vec<&str> = contents.split("\n\n").collect();

    for passport in &all_passport_strings {
        let mut passport_bad = false;

        for field in required_fields.iter() {
            if !passport.contains(field) {
                passport_bad = true;
                println!("Missing {}", field);
                break;
            }
        }

        if passport_bad {
            continue;
        }

        good_passports += 1;
    }

    println!("Good passports: {}/{}", good_passports, all_passport_strings.len()); 
    return good_passports;
}

fn validate_byr(passport: &str) -> bool {
    let re = Regex::new(r"byr:(\d+?)(\s|\n|$)").unwrap();

    for cap in re.captures_iter(passport) {
        let byr = cap[1].parse::<i32>().unwrap();
        return byr >= 1920 && byr <= 2002;
    }

    return false;
}

fn validate_iyr(passport: &str) -> bool {
    let re = Regex::new(r"iyr:(\d+?)(\s|\n|$)").unwrap();

    for cap in re.captures_iter(passport) {
        let byr = cap[1].parse::<i32>().unwrap();
        return byr >= 2010 && byr <= 2020;
    }

    return false;
}

fn validate_eyr(passport: &str) -> bool {
    let re = Regex::new(r"eyr:(\d+?)(\s|\n|$)").unwrap();

    for cap in re.captures_iter(passport) {
        let byr = cap[1].parse::<i32>().unwrap();
        return byr >= 2020 && byr <= 2030;
    }

    return false;
}

fn validate_hgt(passport: &str) -> bool {
    let re = Regex::new(r"hgt:(\d+?)(in|cm)(\s|\n|$)").unwrap();

    for cap in re.captures_iter(passport) {
        let is_cm = cap[2].trim() == "cm";
        let value = cap[1].parse::<i32>().unwrap();

        if is_cm {
            return value >= 150 && value <= 193;
        }
        else {
            return value >= 59 && value <= 76;
        }
    }

    return false;
}

fn validate_hcl(passport: &str) -> bool {
    let re = Regex::new(r"hcl:(.*?)(\s|\n|$)").unwrap();
    let validation = Regex::new(r"#[a-f0-9]{6}").unwrap();

    for cap in re.captures_iter(passport) {
        let hcl = &cap[1];

        if !hcl.starts_with("#") {
            return false;
        }

        let valid = validation.is_match(hcl.trim());
        return valid;
    }

    return false;
}

fn validate_ecl(passport: &str) -> bool {
    let re = Regex::new(r"ecl:(.*?)(\s|\n|$)").unwrap();
    let acceptable_values = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    for cap in re.captures_iter(passport) {
        let ecl = &cap[1];

        for acceptable in acceptable_values.iter() {
            if ecl.contains(acceptable) {
                return true;
            }
        }
    }

    return false;
}

fn validate_pid(passport: &str) -> bool {
    let re = Regex::new(r"pid:(\d+?)(\s|\n|$)").unwrap();

    for cap in re.captures_iter(passport) {
        let pid = &cap[1];

        let valid = pid.trim().chars().count() == 9;
        return valid;
    }

    return false;
}

pub fn part_2(filename: String) -> i32 {
    let contents = fs::read_to_string(filename).expect("Failed to read file!");
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut good_passports = 0;

    let all_passport_strings : Vec<&str> = contents.split("\n\n").collect();

    for passport in &all_passport_strings {
        let mut passport_bad = false;

        for field in required_fields.iter() {
            if !passport.contains(field) {
                passport_bad = true;
                break;
            }

            let valid = match field {
                &"byr" => validate_byr(&passport),
                &"iyr" => validate_iyr(&passport),
                &"eyr" => validate_eyr(&passport),
                &"hgt" => validate_hgt(&passport),
                &"hcl" => validate_hcl(&passport),
                &"ecl" => validate_ecl(&passport),
                &"pid" => validate_pid(&passport),
                     _ => true
            };

            if !valid {
                passport_bad = true;
                break;
            }
        }

        if passport_bad {
            continue;
        }

        good_passports += 1;
    }

    println!("Good passports: {}/{}", good_passports, all_passport_strings.len()); 
    return good_passports;
}
