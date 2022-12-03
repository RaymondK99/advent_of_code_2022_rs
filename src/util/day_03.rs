use std::collections::HashSet;
use std::iter::FromIterator;
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let contents:Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    match part {
        Part::Part1 => part1(contents),
        Part::Part2 => part2(contents)
    }
}

fn get_value(item:&char) -> u32 {
    if item.is_lowercase() {
        *item as u32  - 'a' as u32  + 1
    } else {
        *item as u32 - 'A' as u32  + 27
    }
}

fn part1(contents : Vec<Vec<char>>) -> String {
    contents.iter()
        .map(|content| {
            HashSet::<&char>::from_iter(content[0..content.len()/2].iter())
                    .intersection(&HashSet::<&char>::from_iter(content[content.len()/2..].iter()))
                .into_iter()
                .map(|item| get_value(item))
                .sum::<u32>()
        }).sum::<u32>().to_string()
}

fn part2(contents : Vec<Vec<char>>) -> String {
    contents.chunks(3)
        .into_iter()
        .map( |group| {
            HashSet::<char>::from_iter(group[0].iter().copied())
                .intersection(&HashSet::<char>::from_iter(group[1].iter().copied()))
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&HashSet::<char>::from_iter(group[2].iter().copied()))
                .map(|item| get_value(item))
                .sum::<u32>()
        }).sum::<u32>().to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!("157", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_03.txt");

        assert_eq!("7997", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!("70", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_03.txt");

        assert_eq!("2545", solve(input.to_string(), Part2));
    }
}
