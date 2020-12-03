use std::fs;
use regex::Regex;

struct PasswordPolicy {
    letter: char,
    max: i32,
    min: i32,
}

struct Password {
    policy: PasswordPolicy,
    password: String
}

fn read_passwords(filename : String) -> Vec<Password> {
    let contents = fs::read_to_string(filename).expect("Failed to read file!");
    let mut passwords: Vec<Password> = vec![];
    let re = Regex::new(r"(\d+)\-(\d+) ([A-z]): (.+)").unwrap();

    for line in contents.split("\n") {
        for cap in re.captures_iter(line) {
            println!("Min: {}, Max: {}, Letter: {}, Password: {}", &cap[1], &cap[2], &cap[3], &cap[4]);

            let policy = PasswordPolicy {
                min: cap[1].parse::<i32>().unwrap(),
                max: cap[2].parse::<i32>().unwrap(),
                letter: cap[3].chars().next().unwrap()
            };

            let password = Password {
                policy: policy,
                password: cap[4].to_string()
            };

            passwords.push(password);
        }
    }

    return passwords;
}

fn password_validation(password : Password) -> bool {
    let count : i32 = password.password.matches(password.policy.letter).count() as i32;
    return count >= password.policy.min && count <= password.policy.max;
}

fn password_validation_part2(password : Password) -> bool {
    let first_index : usize = (password.policy.min - 1) as usize;
    let second_index : usize = (password.policy.max - 1) as usize;

    return (password.password.chars().nth(first_index).unwrap() == password.policy.letter) ^
           (password.password.chars().nth(second_index).unwrap() == password.policy.letter);
}

pub fn part_1() {
    let passwords = read_passwords("day2_input".to_string());
    let mut good_passwords = 0;
    let mut bad_passwords = 0;

    for password in passwords {
        if password_validation(password) {
            good_passwords += 1;
            continue;
        }

        bad_passwords += 1;
    }
    
    println!("Good passwords: {}, Bad passwords: {}", good_passwords, bad_passwords);
}

pub fn part_2() {
    let passwords = read_passwords("day2_input".to_string());
    let mut good_passwords = 0;
    let mut bad_passwords = 0;

    for password in passwords {
        if password_validation_part2(password) {
            good_passwords += 1;
            continue;
        }

        bad_passwords += 1;
    }
    
    println!("Good passwords: {}, Bad passwords: {}", good_passwords, bad_passwords);
}
