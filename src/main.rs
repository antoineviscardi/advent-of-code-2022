use std::fs::read_to_string;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let input = read_to_string("assets/day1.txt").unwrap();
    day_1::solve(&input);

    let input = read_to_string("assets/day2.txt").unwrap();
    day_2::solve(&input);

    let input = read_to_string("assets/day3.txt").unwrap();
    day_3::solve(&input);

    println!("\nDay 4");
    let input = read_to_string("assets/day4.txt").unwrap();
    println!("  Part 1: {}", day_4::solve_pt1(&input));
    println!("  Part 2: {}", day_4::solve_pt2(&input));
}
