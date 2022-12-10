#[derive(PartialEq, Clone)]
pub enum Move {
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

pub struct Strategy(Vec<Round>);

impl Strategy {
    pub fn from_input(input: &String, move_strategy: impl Fn(&Move, &char) -> Move) -> Self {
        let rounds = input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut chars = line.chars();
                let elf_move = match chars.next() {
                    Some('A') => Move::Rock,
                    Some('B') => Move::Paper,
                    Some('C') => Move::Scisor,
                    _ => panic!("unexpected elf move when processing line '{}'", line),
                };
                let your_move = match chars.next_back() {
                    Some(c) => move_strategy(&elf_move, &c),
                    _ => panic!("unexpected move when processing line '{}'", line),
                };
                Round {
                    elf_move,
                    your_move,
                }
            })
            .collect();
        Strategy(rounds)
    }

    pub fn get_points(&self) -> u32 {
        self.0.iter().map(|round| round.get_points()).sum()
    }
}

pub fn part_1_strat(_: &Move, c: &char) -> Move {
    match c {
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scisor,
        _ => panic!("expected one of 'X', 'Y', or 'Z'"),
    }
}

pub fn part_2_strat(elf_move: &Move, c: &char) -> Move {
    match c {
        'X' => match elf_move {
            Move::Rock => Move::Scisor,
            Move::Paper => Move::Rock,
            Move::Scisor => Move::Paper,
        },
        'Y' => elf_move.clone(),
        'Z' => match elf_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scisor,
            Move::Scisor => Move::Rock,
        },
        _ => panic!("expected one of 'X', 'Y', or 'Z'"),
    }
}

pub fn solve(input: &String) {
    let strategy = Strategy::from_input(&input, part_1_strat);
    println!("Day 2, Part 1: {}", strategy.get_points());

    let strategy = Strategy::from_input(&input, part_2_strat);
    println!("Day 2, Part 2: {}", strategy.get_points());
}
