use std::collections::HashSet;

fn is_marker(marker: &[u8], len: usize) -> bool {
    if marker.len() != len {
        panic!(
            "a marker should be 4 elements long, received {}",
            marker.len()
        );
    };

    let set: HashSet<&u8> = HashSet::from_iter(marker.iter());
    set.len() == len
}

pub fn find_start_marker(signal: &[u8]) -> usize {
    let marker_len = 4;
    let (i, _marker) = signal
        .windows(marker_len)
        .enumerate()
        .skip_while(|(_, w)| !is_marker(w, marker_len))
        .next()
        .expect("the signal should have a start marker, none found");
    i + marker_len
}

pub fn find_message_marker(signal: &[u8]) -> usize {
    let marker_len = 14;
    let (i, _marker) = signal
        .windows(marker_len)
        .enumerate()
        .skip_while(|(_, w)| !is_marker(w, marker_len))
        .next()
        .expect("the signal should have a start marker, none found");
    i + marker_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_marker() {
        assert!(is_marker(b"abcd", 4));
        assert!(!is_marker(b"abca", 4));
        assert!(!is_marker(b"abbc", 4));
    }

    #[test]
    fn test_find_start_marker() {
        assert_eq!(find_start_marker(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7)
    }
}
