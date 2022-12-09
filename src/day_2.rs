#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scisor,
}

struct Round {
    elf_move: Move,
    your_move: Move,
}

impl Round {
    pub fn get_points(&self) -> u32 {
        let move_points = match self.your_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scisor => 3,
        };

        let result_points = match self.is_win() {
            true => 6,
            false => match self.is_draw() {
                true => 3,
                false => 0,
            },
        };

        return move_points + result_points;
    }

    fn is_win(&self) -> bool {
        match self.elf_move {
            Move::Rock => self.your_move == Move::Paper,
            Move::Paper => self.your_move == Move::Scisor,
            Move::Scisor => self.your_move == Move::Rock,
        }
    }

    fn is_draw(&self) -> bool {
        self.elf_move == self.your_move
    }
}

struct Strategy(Vec<Round>);

impl Strategy {
    pub fn from_input(input: String) -> Self {
        let rounds = input
            .split('\n')
            .map(|line| {
                let chars = line.chars();
                let elf_move = match chars.next() {
                    Some('A') => Move::Rock,
                    Some('B') => Move::Paper,
                    Some('C') => Move::Scisor,
                };
                let your_move = match chars.next_back() {
                    Some('X') => Move::Rock,
                    Some('Y') => Move::Paper,
                    Some('Z') => Move::Scisor,
                };
                Round {
                    elf_move,
                    your_move,
                }
            })
            .collect();
        Strategy(rounds)
    }

    pub fn get_points()
}
