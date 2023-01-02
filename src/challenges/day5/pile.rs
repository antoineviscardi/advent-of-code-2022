use std::iter::repeat;

#[derive(Debug)]
pub struct Pile {
    pub cargo_crates: Vec<char>,
}

impl Pile {
    fn new() -> Self {
        Pile {
            cargo_crates: Vec::new(),
        }
    }

    fn parse_input_line(line: &str) -> Vec<Option<char>> {
        line.chars()
            .skip(1)
            .step_by(4)
            .map(|c| match c {
                ' ' => None,
                _ => Some(c),
            })
            .collect()
    }

    pub fn from_input(input: &str) -> Vec<Self> {
        type Level = Vec<Option<char>>;
        let levels = input
            .split('\n')
            .take_while(|line| !line.starts_with(" 1 "))
            .map(|line| Self::parse_input_line(line))
            .collect::<Vec<Level>>();

        let num_piles = levels.first().unwrap().len();
        let mut piles = repeat(())
            .take(num_piles)
            .map(|_| Pile::new())
            .collect::<Vec<Pile>>();

        for level in levels.into_iter().rev() {
            for (idx, cargo_crate) in level.into_iter().enumerate() {
                if let Some(c) = cargo_crate {
                    piles
                        .get_mut(idx)
                        .expect("number of piles is always equal to the number of items in a level")
                        .cargo_crates
                        .push(c)
                }
            }
        }

        piles
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn get_input() -> String {
        read_to_string("src/challenges/day5/test-input.txt").unwrap()
    }

    #[test]
    fn test_parse_input_line() {
        let input = get_input();
        let input_lines: Vec<&str> = input.split('\n').collect();

        let result = Pile::parse_input_line(input_lines[0]);
        assert_eq!(result, vec![None, Some('D'), None]);

        let result = Pile::parse_input_line(input_lines[1]);
        assert_eq!(result, vec![Some('N'), Some('C'), None]);

        let result = Pile::parse_input_line(input_lines[2]);
        assert_eq!(result, vec![Some('Z'), Some('M'), Some('P')]);
    }

    #[test]
    fn test_from_input() {
        let pile = Pile::from_input(&get_input());
        assert_eq!(pile.len(), 3);
        assert_eq!(pile.get(0).unwrap().cargo_crates, vec!['Z', 'N']);
        assert_eq!(pile.get(1).unwrap().cargo_crates, vec!['M', 'C', 'D']);
        assert_eq!(pile.get(2).unwrap().cargo_crates, vec!['P']);
    }
}
