use std::fs;
use std::collections::HashMap;

struct Group {
    answers: HashMap<char, bool>
}

pub fn part_1(filename: String) {
    let contents = fs::read_to_string(filename).expect("Failed to read file!");
    let group_strings: Vec<&str> = contents.split("\n\n").collect();
    let mut groups = vec![];

    for group_string in group_strings {
        let mut group = Group {
            answers: HashMap::new()
        };

        for c in group_string.chars() {
            if c == '\n' {
                continue;
            }

            group.answers.insert(c, true);
        }

        groups.push(group);
    }

    let mut total = 0;
    for group in groups {
        total += group.answers.keys().len();
    }

    println!("Total {}", total);
}

pub fn part_2(filename: String) {
    println!("");
}
