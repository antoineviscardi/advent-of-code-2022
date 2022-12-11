use std::fs::read_to_string;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    println!("\nDay 1");
    let input = read_to_string("assets/day1.txt").unwrap();
    println!("  Part 1: {}", day_1::solve_pt1(&input));
    println!("  Part 2: {}", day_1::solve_pt2(&input));

    println!("\nDay 2");
    let input = read_to_string("assets/day2.txt").unwrap();
    println!("  Part 1: {}", day_2::solve_pt1(&input));
    println!("  Part 2: {}", day_2::solve_pt2(&input));

    println!("\nDay 3");
    let input = read_to_string("assets/day3.txt").unwrap();
    println!("  Part 1: {}", day_3::solve_pt1(&input));
    println!("  Part 2: {}", day_3::solve_pt2(&input));

    println!("\nDay 4");
    let input = read_to_string("assets/day4.txt").unwrap();
    println!("  Part 1: {}", day_4::solve_pt1(&input));
    println!("  Part 2: {}", day_4::solve_pt2(&input));

    println!("\nDay 5");
    let input = read_to_string("assets/day5.txt").unwrap();
    println!("  Part 1: {}", day_5::solve_pt1(&input));
    println!("  Part 2: {}", day_5::solve_pt2(&input));
}
