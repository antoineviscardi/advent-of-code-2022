use self::section::Section;

mod section;

fn parse_section_pairs(input: &String) -> impl Iterator<Item = (Section, Section)> + '_ {
    input
        .split('\n')
        .map(|pair| pair.split_once(',').unwrap())
        .map(|(a, b)| (Section::from_string(a), Section::from_string(b)))
}

pub fn solve_pt1(input: &String) -> u32 {
    parse_section_pairs(&input)
        .map(|(a, b)| a.has_full_overlap_with(&b))
        .filter(|x| *x)
        .count()
        .try_into()
        .unwrap()
}

pub fn solve_pt2(input: &String) -> u32 {
    parse_section_pairs(&input)
        .map(|(a, b)| a.has_overlap_with(&b))
        .filter(|x| *x)
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod solve_pt1 {
        use super::*;

        #[test]
        fn it_should_generate_the_correct_result() {
            let input = String::from("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");
            let result = solve_pt1(&input);
            assert_eq!(result, 2);
        }
    }

    mod solve_pt2 {
        use super::*;

        #[test]
        fn it_should_generate_the_correct_result() {
            let input = String::from("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");
            let result = solve_pt2(&input);
            assert_eq!(result, 4);
        }
    }
}
