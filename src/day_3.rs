#[derive(Clone, PartialEq, Debug)]
pub struct Item(char);

impl Item {
    pub fn get_priority(&self) -> u32 {
        let decimal_value: u32 = self.0.into();
        if self.0 >= 'a' && self.0 <= 'z' {
            decimal_value - 96
        } else if self.0 >= 'A' && self.0 <= 'Z' {
            decimal_value - 38
        } else {
            panic!("expected item to be [a-zA-Z]")
        }
    }
}

pub struct Rucksack {
    compartment_1: Vec<Item>,
    compartment_2: Vec<Item>,
}

impl Rucksack {
    pub fn from_input_line(line: &str) -> Self {
        let items: Vec<Item> = line.chars().map(Item).collect();
        let middle = items.len() / 2;
        let (first_half, second_half) = items.split_at(middle);

        Rucksack {
            compartment_1: first_half.to_vec(),
            compartment_2: second_half.to_vec(),
        }
    }

    pub fn get_duplicate_item(&self) -> &Item {
        for item in self.compartment_1.iter() {
            if self.compartment_2.contains(item) {
                return item;
            }
        }
        panic!("No duplicate item found")
    }
}
