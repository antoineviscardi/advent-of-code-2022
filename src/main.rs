mod day_1;
mod day_2;

fn main() {
    let elves = day_1::app::parse_file("assets/day1.txt".to_string());
    let result = day_1::app::get_top_n_elves_calories(&elves, 1);
    println!("Day 1, Part 1: {:?}", result);

    let result = day_1::app::get_top_n_elves_calories(&elves, 3);
    println!("Day 1, Part 2: {:?}", result);
}
