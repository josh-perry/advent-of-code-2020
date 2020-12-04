//mod day_1;
//mod day_2;
mod day_3;

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
}
