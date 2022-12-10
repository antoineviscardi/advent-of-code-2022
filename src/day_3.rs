#[derive(Clone, PartialEq, Debug)]
struct Item(char);

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

struct Rucksack {
    items: Vec<Item>,
}

impl Rucksack {
    pub fn from_input_line(line: &str) -> Self {
        let items: Vec<Item> = line.chars().map(Item).collect();
        Rucksack { items }
        // let middle = items.len() / 2;
        // let (first_half, second_half) = items.split_at(middle);

        // Rucksack {
        //     compartment_1: first_half.to_vec(),
        //     compartment_2: second_half.to_vec(),
        // }
    }

    fn get_compartments(&self) -> (&[Item], &[Item]) {
        let middle = self.items.len() / 2;
        self.items.split_at(middle)
    }

    pub fn get_duplicate_item(&self) -> &Item {
        let (compartment_1, compartment_2) = self.get_compartments();
        for item in compartment_1.iter() {
            if compartment_2.contains(item) {
                return item;
            }
        }
        panic!("No duplicate item found")
    }
}

struct Group<'a> {
    pub rucksacks: &'a [Rucksack],
}

impl Group<'_> {
    pub fn get_badge_item(&self) -> &Item {
        for item in &self.rucksacks[0].items {
            if self.rucksacks[1..]
                .iter()
                .all(|ruck| ruck.items.contains(item))
            {
                return item;
            }
        }
        panic!("no badge found for group")
    }
}

pub fn solve(input: &String) {
    let rucksacks: Vec<Rucksack> = input
        .split('\n')
        .map(|line| Rucksack::from_input_line(line))
        .collect();

    let result: u32 = rucksacks
        .iter()
        .map(|r| r.get_duplicate_item())
        .map(|i| i.get_priority())
        .sum();
    println!("Day 3, Part 1: {}", result);

    let result_pt2: u32 = rucksacks
        .chunks(3)
        .map(|chunk| Group { rucksacks: chunk })
        .map(|group| group.get_badge_item().get_priority())
        .sum();
    println!("Day 3, Part 2: {}", result_pt2);
}
