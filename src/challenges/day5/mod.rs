use pile::Pile;

use self::movement::Movement;

mod movement;
mod pile;

pub fn solve_pt1(input: &str) -> String {
    let mut piles = Pile::from_input(input);
    let movements = parse_movements(input);

    for movement in movements.iter() {
        movement.execute_crane_9000(&mut piles);
    }

    let result = piles
        .iter()
        .map(|pile| pile.cargo_crates.last().unwrap())
        .collect::<String>();

    result
}

pub fn solve_pt2(input: &str) -> String {
    let mut piles = Pile::from_input(input);
    let movements = parse_movements(input);

    for movement in movements.iter() {
        movement.execute_crane_9001(&mut piles);
    }

    let result = piles
        .iter()
        .map(|pile| pile.cargo_crates.last().unwrap())
        .collect::<String>();

    result
}

fn parse_movements(input: &str) -> Vec<Movement> {
    input
        .split('\n')
        .skip_while(|line| !line.starts_with("move"))
        .take_while(|line| line.starts_with("move"))
        .map(|line| Movement::parse(line))
        .collect::<Vec<Movement>>()
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

    #[test]
    fn test_solve_pt2() {
        assert_eq!(solve_pt2(&get_input()), "MCD");
    }
}
