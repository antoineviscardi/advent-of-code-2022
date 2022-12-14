mod signal;

pub fn solve_pt1(input: &str) -> usize {
    signal::find_start_marker(input.as_bytes())
}

pub fn solve_pt2(input: &str) -> usize {
    signal::find_message_marker(input.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve_pt1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(solve_pt1(input), 5);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(solve_pt1(input), 6);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(solve_pt1(input), 10);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(solve_pt1(input), 11);
    }

    #[test]
    pub fn test_solve_pt2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solve_pt2(input), 19);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(solve_pt2(input), 23);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(solve_pt2(input), 23);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(solve_pt2(input), 29);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(solve_pt2(input), 26);
    }
}
