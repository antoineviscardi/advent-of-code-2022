struct Section {
    start: u32,
    end: u32,
}

impl Section {
    pub fn from_input_string(string: String) -> Self {
        let (start, end) = string.split_once('-').expect("expec");
        let start: u32 = start.parse().unwrap();
        let end: u32 = end.parse().unwrap();
        Section { start, end }
    }
}
