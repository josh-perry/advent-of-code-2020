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

fn read_numbers(filename : String) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("Failed to read file!");
    let mut numbers : Vec<i32> = vec![];

    for line in contents.split_whitespace() {
        numbers.push(line.parse::<i32>().unwrap());
    }

    return numbers;
}

fn find_pair(numbers : &[i32]) -> i32 {
    let target = 2020;

    for &number in numbers {
        for &number2 in numbers {
            if &number + number2 == target {
                return number*number2;
            }
        }
    }

    return -1;
}

fn find_triplet(numbers : &[i32]) -> i32 {
    let target = 2020;

    for &number in numbers {
        for &number2 in numbers {
            for &number3 in numbers {
                if number + number2 + number3 == target {
                    return number*number2*number3;
                }
            }
        }
    }

    return -1;
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

fn day1() {
    let numbers = read_numbers("day1_input".to_string());

    println!("Pair:    {}", find_pair(&numbers));
    println!("Triplet: {}", find_triplet(&numbers));
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

fn day2() {
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

fn day2_part2() {
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

fn main() {
    day1();
    day2();
    day2_part2();
}
