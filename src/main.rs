use std::fs::read_to_string;

mod day_1;
mod day_2;

fn main() {
    // Day 1
    let elves = day_1::app::parse_file("assets/day1.txt".to_string());
    let result = day_1::app::get_top_n_elves_calories(&elves, 1);
    println!("Day 1, Part 1: {:?}", result);

    let result = day_1::app::get_top_n_elves_calories(&elves, 3);
    println!("Day 1, Part 2: {:?}", result);

    // Day 2
    let input = read_to_string("assets/day2.txt").unwrap();
    let strategy = day_2::Strategy::from_input(input);
    println!("Day 2, Part 1: {}", strategy.get_points());
}
