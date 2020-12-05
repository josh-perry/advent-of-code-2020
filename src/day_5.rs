use std::fs;
use std::cmp;

pub fn find_seat(pass: &str, rows: f32, cols: f32) -> i32 {
    let mut current_min_row = 0.0;
    let mut current_max_row = rows;

    let mut current_min_col = 0.0;
    let mut current_max_col = cols;

    for c in pass.chars() {
        println!("Row: {}, {}", current_min_row, current_max_row);
        println!("Col: {}, {}", current_min_col, current_max_col);

        match c {
            'F' => current_max_row = ((current_min_row + current_max_row) / 2.0).floor(), 
            'B' => current_min_row = ((current_min_row + current_max_row) / 2.0).ceil(),
            'L' => current_max_col = ((current_min_col + current_max_col) / 2.0).floor(), 
            'R' => current_min_col = ((current_min_col + current_max_col) / 2.0).ceil(),
              _ => panic!("Unrecognised character: {}", c)
        }
    }

    println!("{}, {}", current_min_row, current_max_row);
    println!("{}, {}", current_min_col, current_max_col);

    return (current_min_row * 8.0 + current_min_col) as i32;
}

pub fn part_1(filename: String) -> i32 {
    let contents = fs::read_to_string(filename).expect("Failed to read file!");
    let mut seats = vec![];

    for line in contents.split("\n") {
        seats.push(find_seat(line, 127.0, 7.0));
    }

    let mut biggest = 0;
    for seat in seats {
        biggest = cmp::max(biggest, seat);
    }
    
    println!("Highest seat number: {}", biggest);
    return biggest;
}

pub fn part_2(filename: String) -> i32 {
    let contents = fs::read_to_string(filename).expect("Failed to read file!");
    let mut seats = vec![];

    for line in contents.split("\n") {
        let seat = find_seat(line, 127.0, 7.0);

        if seat == 0 {
            println!("{}", line);
        }

        seats.push(seat);
    }

    seats.sort();

    let min = seats[0];
    let max = seats[seats.len() - 1];

    // Loop over them all
    for c in min..max {
        // If a seat is missing, check 1 to the left and 1 to the right, if they exist then
        // we've found our seat!
        if !seats.iter().any(|&i| i == c) {
            if seats.iter().any(|&i| i == c + 1) && seats.iter().any(|&i| i == c - 1) {
                println!("Missing seat is {}", c);
                return c;
            }
        }
    }

    panic!("Failed to find seat!");
}
