use std::{collections::HashSet, fs::File, io::Read};

const INPUT_FILE: &str = "input_file.txt";
const MARKER_LENGTH: usize = 4;

fn main() {
    let mut input_file = File::open(INPUT_FILE).unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    let marker = find_marker(&input);
    println!("start marker position: {marker}");
}

fn find_marker(input: &str) -> usize {
    let input_array = input.as_bytes();
    let mut set = HashSet::new();

    for i in 0..input_array.len() {
        (i..i + MARKER_LENGTH).for_each(|j| {
            set.insert(input_array[j]);
        });

        if set.len() == MARKER_LENGTH {
            return i + MARKER_LENGTH;
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
        let marker = find_marker(input);
        assert_eq!(marker, 7)
    }

    #[test]
    fn test2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = find_marker(input);
        assert_eq!(marker, 5)
    }

    #[test]
    fn test3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = find_marker(input);
        assert_eq!(marker, 6)
    }

    #[test]
    fn test4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = find_marker(input);
        assert_eq!(marker, 10)
    }

    #[test]
    fn test5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = find_marker(input);
        assert_eq!(marker, 11)
    }
}
