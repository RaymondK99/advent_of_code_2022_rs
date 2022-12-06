use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

fn find_marker(input : String, num_chars:usize) -> String {
    let buffer = input.as_bytes();

    for i in 0..buffer.len() - num_chars {
        let unique_symbols = buffer[i..(i+num_chars)].iter()
            .fold(0 as u32, |acc,next| acc | (1 << (*next - 'a' as u8)))
            .count_ones();

        if unique_symbols == num_chars as u32 {
            return (i+num_chars).to_string()
        }
    }

    panic!("No solution");
}

fn part1(input : String) -> String {
    find_marker(input, 4)
}

fn part2(input : String) -> String {
    find_marker(input, 14)
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let input2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let input3 = "nppdvjthqldpwncqszvftbrmjlhg";
        let input4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        assert_eq!("7", solve(input1.to_string(), Part1));
        assert_eq!("5", solve(input2.to_string(), Part1));
        assert_eq!("6", solve(input3.to_string(), Part1));
        assert_eq!("10", solve(input4.to_string(), Part1));

    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_06.txt");

        assert_eq!("1855", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let input2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let input3 = "nppdvjthqldpwncqszvftbrmjlhg";
        let input4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        assert_eq!("19", solve(input1.to_string(), Part2));
        assert_eq!("23", solve(input2.to_string(), Part2));
        assert_eq!("23", solve(input3.to_string(), Part2));
        assert_eq!("29", solve(input4.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_06.txt");

        assert_eq!("3256", solve(input.to_string(), Part2));
    }
}
