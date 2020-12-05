mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    println!("Day 1");
    day_1::part_1();
    day_1::part_2();

    println!("Day 2");
    day_2::part_1();
    day_2::part_2();

    println!("Day 3");
    day_3::part_1("day3_input".to_string());
    day_3::part_2("day3_input".to_string());
    
    println!("Day 4");
    day_4::part_1("day4_input".to_string());
    day_4::part_2("day4_input".to_string());
    
    println!("Day 5");
    day_5::part_1("day5_input".to_string());
    day_5::part_2("day5_input".to_string());
}
