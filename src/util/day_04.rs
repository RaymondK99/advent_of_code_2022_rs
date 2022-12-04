use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn parse(line:&str) -> ((u8,u8),(u8,u8)) {
    let numbers = line.split([',','-'])
        .into_iter()
        .map(|item| item.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    ((numbers[0], numbers[1]), (numbers[2], numbers[3]))
}

fn is_subset(ranges:&((u8,u8),(u8, u8))) -> bool {
    let ((s1,e1),(s2,e2)) = ranges;
    s1 >= s2 && e1 <= e2 || s2 >= s1 && e2 <= e1
}

fn is_disjoint(ranges:&((u8,u8),(u8, u8))) -> bool {
    let ((s1,e1),(s2,e2)) = ranges;
    s1 > e2 || s2 > e1
}

fn count_ranges(lines : Vec<&str>, lambda: fn(&((u8, u8), (u8, u8))) -> bool) -> String {
    lines.iter()
        .map(|line| parse(line))
        .filter(|ranges | lambda(ranges))
        .count()
        .to_string()
}

fn part1(lines : Vec<&str>) -> String {
    count_ranges(lines, is_subset)
}

fn part2(lines : Vec<&str>) -> String {
    count_ranges(lines, |arg| !is_disjoint(arg))
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!("2", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_04.txt");

        assert_eq!("503", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!("4", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_04.txt");

        assert_eq!("827", solve(input.to_string(), Part2));
    }
}
