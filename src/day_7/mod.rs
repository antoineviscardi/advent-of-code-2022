mod directory;

pub fn solve_pt1(input: &str) -> u32 {
    todo!()
}

pub fn solve_pt2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_solve_pt1() {
        let input = read_to_string("src/day_7/test-input.txt").unwrap();
        let result = solve_pt1(&input);
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_solve_pt2() {}
}
