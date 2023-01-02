use advent_of_code_2022::challenges::{day1, day2, day3, day4, day5, day6, day7};

fn main() {
    println!("AoC 2022");

    let day = parse_day_from_args();
    println!("Day {}", day.0);
    let input = std::fs::read_to_string(format!("assets/day{}.txt", day.0)).unwrap();
    let (pt1, pt2) = match day.0 {
        1 => (day1::solve_pt1(&input), day1::solve_pt2(&input)),
        2 => (day2::solve_pt1(&input), day2::solve_pt2(&input)),
        3 => (day3::solve_pt1(&input), day3::solve_pt2(&input)),
        4 => (day4::solve_pt1(&input), day4::solve_pt2(&input)),
        5 => (day5::solve_pt1(&input), day5::solve_pt2(&input)),
        6 => (day6::solve_pt1(&input), day6::solve_pt2(&input)),
        7 => (day7::solve_pt1(&input), day7::solve_pt2(&input)),
        _ => panic!("day number should be between 1 and 25, got {}", day.0),
    };
    println!("  Part 1: {}", pt1);
    println!("  Part 2: {}", pt2);
}

struct Day(u8);
fn parse_day_from_args() -> Day {
    let mut args = std::env::args();

    // Skiping 0 as the first arg is the executable name
    let arg = args
        .nth(1)
        .expect("should be called with an argument, none found");

    let invalid_arg_msg =
        "the argument should have the form `<#>day` where `<#>` is a number between 1 and 25";

    let len = arg.len();
    if !(len >= 4 && len <= 5) {
        panic!("{invalid_arg_msg}")
    };

    if &arg[..3] != "day" {
        panic!("{invalid_arg_msg}")
    };

    let n = arg[3..len]
        .parse()
        .unwrap_or_else(|_| panic!("{invalid_arg_msg}"));

    return Day(n);
}
