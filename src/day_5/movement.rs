use super::pile::Pile;

pub struct Movement {
    quantity: u32,
    from: usize,
    to: usize,
}

impl Movement {
    pub fn parse(string: &str) -> Self {
        let words = string.split(' ').collect::<Vec<&str>>();
        Movement {
            quantity: words.get(1).unwrap().parse().unwrap(),
            from: words.get(3).unwrap().parse().unwrap(),
            to: words.get(5).unwrap().parse().unwrap(),
        }
    }

    pub fn execute_crane_9000(&self, piles: &mut Vec<Pile>) {
        for _ in 0..self.quantity {
            let moving_crate = piles
                .get_mut(self.from - 1)
                .and_then(|pile| pile.cargo_crates.pop())
                .unwrap();
            piles
                .get_mut(self.to - 1)
                .and_then(|pile| Some(pile.cargo_crates.push(moving_crate)))
                .unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let string = "move 3 from 1 to 3";
        let result = Movement::parse(string);

        assert_eq!(result.quantity, 3);
        assert_eq!(result.from, 1);
        assert_eq!(result.to, 3);
    }

    #[test]
    fn test_execute_crane_9000() {
        let pile1 = Pile {
            cargo_crates: vec!['A', 'B', 'C'],
        };
        let pile2 = Pile {
            cargo_crates: vec!['X', 'Y', 'Z'],
        };
        let mut piles = vec![pile1, pile2];
        let movement = Movement::parse("move 2 from 1 to 2");

        movement.execute_crane_9000(&mut piles);

        assert_eq!(piles.get(0).unwrap().cargo_crates, vec!['A']);
        assert_eq!(
            piles.get(1).unwrap().cargo_crates,
            vec!['X', 'Y', 'Z', 'C', 'B']
        );
    }
}
