use std::collections::HashSet;

fn find_marker(input: &str, unique_chars: usize) -> usize {
    input
        .as_bytes()
        .windows(unique_chars)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == unique_chars)
        .unwrap()
        + unique_chars
}

pub fn d06(input: &str) -> (usize, usize) {
    let part1 = find_marker(input, 4);
    let part2 = find_marker(input, 14);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_4() {
        for (result, input) in [
            (5, "bvwbjplbgvbhsrlpgdmjqwftvncz"),
            (6, "nppdvjthqldpwncqszvftbrmjlhg"),
            (10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            (11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        ] {
            assert_eq!(result, super::find_marker(input, 4));
        }
    }
    #[test]
    fn test_14() {
        for (result, input) in [
            (19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            (23, "bvwbjplbgvbhsrlpgdmjqwftvncz"),
            (23, "nppdvjthqldpwncqszvftbrmjlhg"),
            (29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            (26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        ] {
            assert_eq!(result, super::find_marker(input, 14));
        }
    }
}
