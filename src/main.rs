use std::fs;

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

fn main() {
    let numbers = read_numbers("input".to_string());

    println!("Pair:    {}", find_pair(&numbers));
    println!("Triplet: {}", find_triplet(&numbers));
}
