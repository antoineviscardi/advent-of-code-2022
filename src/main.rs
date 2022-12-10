use std::fs::read_to_string;

mod day_1;
mod day_2;
mod day_3;

fn main() {
    // Day 1
    let elves = day_1::app::parse_file("assets/day1.txt".to_string());
    let result = day_1::app::get_top_n_elves_calories(&elves, 1);
    println!("Day 1, Part 1: {:?}", result);

    let result = day_1::app::get_top_n_elves_calories(&elves, 3);
    println!("Day 1, Part 2: {:?}", result);

    // Day 2
    let input = read_to_string("assets/day2.txt").unwrap();
    let strategy = day_2::Strategy::from_input(&input, day_2::part_1_strat);
    println!("Day 2, Part 1: {}", strategy.get_points());

    let strategy = day_2::Strategy::from_input(&input, day_2::part_2_strat);
    println!("Day 2, Part 2: {}", strategy.get_points());

    // Day 3
    let input = read_to_string("assets/day3.txt").unwrap();
    let rucksacks = input
        .split('\n')
        .map(|line| day_3::Rucksack::from_input_line(line));
    let result: u32 = rucksacks
        .flat_map(|rucksack| rucksack.get_duplicate_items())
        .map(|item| item.get_priority())
        .sum();
}
