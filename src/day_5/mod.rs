use pile::Pile;

use self::movement::Movement;

mod movement;
mod pile;

pub fn solve_pt1(input: &str) -> String {
    let mut piles = Pile::from_input(input);

    let movements = input
        .split('\n')
        .skip_while(|line| !line.starts_with("move"))
        .take_while(|line| line.starts_with("move"))
        .map(|line| Movement::parse(line))
        .collect::<Vec<Movement>>();

    for movement in movements.iter() {
        movement.execute_crane_9000(&mut piles);
    }

    let result = piles
        .iter()
        .map(|pile| pile.cargo_crates.last().unwrap())
        .collect::<String>();

    result
}

pub fn solve_pt2(input: &str) -> &str {
    "TODO"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn get_input() -> String {
        read_to_string("src/day_5/test-input.txt").unwrap()
    }

    #[test]
    fn test_solve_pt1() {
        assert_eq!(solve_pt1(&get_input()), "CMZ");
    }
}
