pub struct Section {
    start: u32,
    end: u32,
}

impl Section {
    pub fn from_string(section_str: &str) -> Self {
        let gen_err_msg = || {
            format!(
                "section string must have the format '<int>-<int>', got '{}'",
                section_str
            )
        };

        let (start, end) = section_str.split_once('-').expect(&gen_err_msg());
        Section {
            start: start.parse().expect(&gen_err_msg()),
            end: end.parse().expect(&gen_err_msg()),
        }
    }

    pub fn has_full_overlap_with(&self, other: &Section) -> bool {
        (other.start <= self.start && other.end >= self.end)
            || (self.start <= other.start && self.end >= other.end)
    }

    pub fn has_overlap_with(&self, other: &Section) -> bool {
        (self.start.ge(&other.start) && self.start.le(&other.end))
            || (self.end.ge(&other.start) && self.end.le(&other.end))
            || (self.start.le(&other.start) && self.end.ge(&other.end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_input_string {
        use super::*;

        #[test]
        fn it_should_properly_parse_the_string() {
            let s = "42-69";
            let section = Section::from_string(s);
            assert_eq!(section.start, 42);
            assert_eq!(section.end, 69);
        }
    }

    mod has_full_overlap_with {
        use super::*;

        #[test]
        fn it_should_return_true_if_other_fully_overlaps_self() {
            let section = Section { start: 5, end: 8 };
            let other = Section { start: 2, end: 12 };
            assert!(section.has_full_overlap_with(&other))
        }

        #[test]
        fn it_should_return_true_if_self_fully_overlaps_other() {
            let section = Section { start: 2, end: 42 };
            let other = Section { start: 10, end: 20 };
            assert!(section.has_full_overlap_with(&other))
        }

        #[test]
        fn it_should_return_false_if_there_is_no_full_overlap() {
            let section = Section { start: 4, end: 8 };
            let other = Section { start: 2, end: 6 };
            assert!(!section.has_full_overlap_with(&other))
        }
    }

    mod has_overlap_with {
        use super::*;

        #[test]
        fn it_should_return_true_if_there_id_overlap() {
            let section = Section { start: 10, end: 20 };
            assert!(section.has_overlap_with(&Section { start: 5, end: 15 }));
            assert!(section.has_overlap_with(&Section { start: 15, end: 25 }));
            assert!(section.has_overlap_with(&Section { start: 10, end: 12 }));
            assert!(section.has_overlap_with(&Section { start: 5, end: 25 }));
            assert!(section.has_overlap_with(&Section { start: 5, end: 10 }));
            assert!(section.has_overlap_with(&Section { start: 20, end: 25 }));
            assert!(section.has_overlap_with(&Section { start: 15, end: 15 }));
        }

        #[test]
        fn it_should_return_false_if_there_is_no_overlap() {
            let section = Section { start: 10, end: 20 };
            assert!(!section.has_overlap_with(&Section { start: 1, end: 8 }));
            assert!(!section.has_overlap_with(&Section { start: 21, end: 30 }));
        }
    }
}
