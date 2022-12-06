use std::{collections::HashSet, fs::File, io::Read};

const INPUT_FILE: &str = "input_file.txt";
const START_PACKAGE_MARKER_LENGTH: usize = 4;
const START_MESSAGE_MARKER_LENGTH: usize = 14;

fn main() {
    let mut input_file = File::open(INPUT_FILE).unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    let start_packet_marker = find_start_marker(&input, START_PACKAGE_MARKER_LENGTH);
    let start_message_marker = find_start_marker(&input, START_MESSAGE_MARKER_LENGTH);
    println!("start package marker position: {start_packet_marker}");
    println!("start message marker position: {start_message_marker}")
}

fn find_start_marker(input: &str, marker_lenght: usize) -> usize {
    let input_array = input.as_bytes();
    let mut set = HashSet::new();

    for i in 0..input_array.len() {
        (i..i + marker_lenght).for_each(|j| {
            set.insert(input_array[j]);
        });

        if set.len() == marker_lenght {
            return i + marker_lenght;
        } else {
            set.clear()
        }
    }
    panic!("cannot find marker")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let marker = find_start_marker(input, START_PACKAGE_MARKER_LENGTH);
        assert_eq!(marker, 7)
    }

    #[test]
    fn test2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = find_start_marker(input, START_PACKAGE_MARKER_LENGTH);
        assert_eq!(marker, 5)
    }

    #[test]
    fn test3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = find_start_marker(input, START_PACKAGE_MARKER_LENGTH);
        assert_eq!(marker, 6)
    }

    #[test]
    fn test4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = find_start_marker(input, START_PACKAGE_MARKER_LENGTH);
        assert_eq!(marker, 10)
    }

    #[test]
    fn test5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = find_start_marker(input, START_PACKAGE_MARKER_LENGTH);
        assert_eq!(marker, 11)
    }

    #[test]
    fn test6() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let marker = find_start_marker(input, START_MESSAGE_MARKER_LENGTH);
        assert_eq!(marker, 19)
    }
    #[test]
    fn test7() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = find_start_marker(input, START_MESSAGE_MARKER_LENGTH);
        assert_eq!(marker, 23)
    }
    #[test]
    fn test8() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = find_start_marker(input, START_MESSAGE_MARKER_LENGTH);
        assert_eq!(marker, 23)
    }
    #[test]
    fn test9() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = find_start_marker(input, START_MESSAGE_MARKER_LENGTH);
        assert_eq!(marker, 29)
    }
    #[test]
    fn test10() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = find_start_marker(input, START_MESSAGE_MARKER_LENGTH);
        assert_eq!(marker, 26)
    }
}
