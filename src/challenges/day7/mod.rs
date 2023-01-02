mod filesystem;

pub fn solve_pt1(input: &str) -> String {
    for line in input.split('\n') {
        println!("{line}");
    }

    String::from("")
}

pub fn solve_pt2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_solve_pt1() {
        let input = read_to_string("src/challenges/day7/test-input.txt").unwrap();
        let result = solve_pt1(&input);
        assert_eq!(result, "95437");
    }

    #[test]
    fn test_solve_pt2() {}
}
