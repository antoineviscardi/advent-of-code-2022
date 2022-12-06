pub mod app {
    use super::domain;
    use std::fs::read_to_string;

    pub fn process_file(path: String) -> Vec<domain::Elf> {
        let file_content = read_to_string(path).unwrap();

        let result = Vec::from('a');
        file_content.split('\n').for_each(|line| {
            result
        })

        return result;
    }
}

mod domain {
    pub struct Elf {
        inventory: Vec<Food>,
    }

    impl Elf {
        fn total_calories(&self) -> u32 {
            self.inventory.iter().map(|f| f.calories).sum()
        }
    }

    struct Food {
        calories: u32,
    }
}
