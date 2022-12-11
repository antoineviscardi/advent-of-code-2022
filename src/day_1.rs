pub mod app {
    use super::domain::{Elf, Food};

    pub fn parse_file(input: &String) -> Vec<Elf> {
        let mut result = Vec::new();
        result.push(Elf::new());

        input.split('\n').for_each(|line| {
            if line == "" {
                result.push(Elf::new());
                return;
            };

            result.last_mut().unwrap().inventory.push(Food {
                calories: line.parse().unwrap(),
            });
        });

        return result;
    }

    pub fn get_top_n_elves_calories(elves: &Vec<Elf>, n: usize) -> u32 {
        let mut elves_calories: Vec<u32> = elves
            .clone()
            .iter()
            .map(|elf| elf.total_calories())
            .collect();
        elves_calories.sort_by(|a, b| b.cmp(a));
        elves_calories.truncate(n);
        elves_calories.iter().sum()
    }
}

mod domain {

    #[derive(Debug)]
    pub struct Elf {
        pub inventory: Vec<Food>,
    }

    impl Elf {
        pub fn new() -> Self {
            Elf {
                inventory: Vec::new(),
            }
        }

        pub fn total_calories(&self) -> u32 {
            self.inventory.iter().map(|f| f.calories).sum()
        }
    }

    #[derive(Debug)]
    pub struct Food {
        pub calories: u32,
    }
}

pub fn solve_pt1(input: &String) -> u32 {
    let elves = app::parse_file(input);
    app::get_top_n_elves_calories(&elves, 1)
}

pub fn solve_pt2(input: &String) -> u32 {
    let elves = app::parse_file(input);
    app::get_top_n_elves_calories(&elves, 3)
}
