mod day_1;

fn main() {
    let elfs = day_1::app::parse_file("assets/day1.txt".to_string());
    let result = day_1::app::get_top_elf_calories(&elfs);
    println!("Day 1, Part 1: {:?}", result)
}
