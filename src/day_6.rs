use std::fs;
use std::collections::HashMap;

struct Group {
    answers: HashMap<char, bool>,
    individual_answers: Vec<HashMap<char, bool>>
}

fn get_groups(group_strings: Vec<&str>) -> Vec<Group> {
    let mut groups = vec![];

    for group_string in group_strings {
        let mut group = Group {
            answers: HashMap::new(),
            individual_answers: vec![]
        };

        for individual_string in group_string.trim().split("\n") {
            let mut individual_answers: HashMap<char, bool> = HashMap::new();

            for c in individual_string.chars() {
                individual_answers.insert(c, true);
                group.answers.insert(c, true);
            }

            group.individual_answers.push(individual_answers);
        }

        groups.push(group);
    }

    return groups;
}

pub fn part_1(filename: String) {
    let contents = fs::read_to_string(filename).expect("Failed to read file!");
    let group_strings: Vec<&str> = contents.split("\n\n").collect();
    let groups = get_groups(group_strings);

    let mut total = 0;
    for group in groups {
        total += group.answers.keys().len();
    }

    println!("Total {}", total);
}

pub fn part_2(filename: String) {
    let contents = fs::read_to_string(filename).expect("Failed to read file!");
    let group_strings: Vec<&str> = contents.split("\n\n").collect();
    let groups = get_groups(group_strings);

    let mut total = 0;
    // Loop over all groups
    for group in groups {
        // Loop over all the group answers as CHARS
        for (k, v) in group.answers.iter() {
            // Check each individual, if any of them don't contain v then break
            // If they all contain v then add 1 to total
            let mut bad = false;

            for individual in &group.individual_answers {
                if !individual.contains_key(k) {
                    bad = true;
                    break;
                }
            }

            if !bad {
                total += 1;
            }
        }
    }

    println!("Total {}", total);
}
