pub mod app {
    use super::domain::{Elf, Food};
    use std::fs::read_to_string;

    pub fn parse_file(path: String) -> Vec<Elf> {
        let file_content = read_to_string(path).unwrap();

        let mut result = Vec::new();
        result.push(Elf::new());

        file_content.split('\n').for_each(|line| {
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

    pub fn get_top_elf_calories(elfs: &Vec<Elf>) -> u32 {
        elfs.iter().map(|elf| elf.total_calories()).max().unwrap()
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
