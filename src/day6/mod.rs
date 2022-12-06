use std::collections::HashSet;

fn start_of_packet_index(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    first_window_with_all_different_characters(chars.as_slice(), 4)
}

fn start_of_message_index(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    first_window_with_all_different_characters(chars.as_slice(), 14)
}

fn first_window_with_all_different_characters(signal: &[char], window_size: usize) -> usize {
    signal
        .windows(window_size)
        .enumerate()
        .find_map(|(idx, window_of_chars)| {
            if window_of_chars.iter().cloned().collect::<HashSet<char>>().len() == window_size {
                Some(idx + window_size - 1)
            } else {
                None
            }
        }).unwrap()
}


#[cfg(test)]
mod tests {
    use crate::day6::{start_of_message_index, start_of_packet_index};

    #[test]
    fn test_example1() {
        let input = include_str!("example.txt");

        assert_eq!(
            start_of_packet_index(input) + 1,
            7
        );
    }

    #[test]
    fn test_input1() {
        let input = include_str!("input.txt");

        assert_eq!(
            start_of_packet_index(input) + 1,
            1598
        );
    }

    #[test]
    fn test_example2() {
        let input = include_str!("example.txt");

        assert_eq!(
            start_of_message_index(input) + 1,
            19
        );
    }

    #[test]
    fn test_input2() {
        let input = include_str!("input.txt");

        assert_eq!(
            start_of_message_index(input) + 1,
            2414
        );
    }
}