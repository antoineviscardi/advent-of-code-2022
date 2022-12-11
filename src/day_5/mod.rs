pub fn solve_pt1(input: &str) -> &str {
    input
}

pub fn solve_pt2(input: &str) -> &str {
    "TODO"
}

pub struct Pile {
    cargo_crates: Vec<char>,
}

impl Pile {
    pub fn from_input(input: &str) -> Vec<Self> {
        let lines = input
            .split('\n')
            .take_while(|line| line.starts_with('[') || line.starts_with("  "));
    }
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
    fn test_pile_from_input() {
        let pile = Pile::from_input(&get_input());
        assert_eq!(pile.len(), 3);
        assert_eq!(pile.get(0).unwrap().cargo_crates, vec!['Z', 'N']);
        assert_eq!(pile.get(1).unwrap().cargo_crates, vec!['M', 'C', 'D']);
        assert_eq!(pile.get(2).unwrap().cargo_crates, vec!['P']);
    }
}
