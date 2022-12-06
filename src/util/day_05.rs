use std::collections::VecDeque;
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn parse_moves(lines:&Vec<&str>) -> Vec<(usize,usize,usize)> {
    lines.iter()
        .filter( |line| line.contains("move"))
        .map(|line| {
            let v:Vec<&str> = line.split(' ').collect();
            let n = v[1].parse::<usize>().unwrap();
            let from = v[3].parse::<usize>().unwrap();
            let to = v[5].parse::<usize>().unwrap();
            (n, from-1, to-1)
        })
        .collect::<Vec<_>>()
}

fn parse_map(lines:&Vec<&str>) -> Vec<VecDeque<char>> {
    let stack_lines = lines.iter()
        .filter( |line| line.contains("["))
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let num_cols = stack_lines.iter().map(|line| (line.len()-1)/4).max().unwrap();

    let mut stacks = vec![];
    (0..=num_cols).into_iter().for_each(|_| stacks.push(VecDeque::new()));

    stack_lines.iter()
        .for_each(|stack_line| {
            stack_line.iter().enumerate()
                .filter(|(_,ch)| ch.is_ascii_alphabetic())
                .for_each(|(pos, ch)| {
                    let col_no = (pos-1)/4;
                    let stack = stacks.get_mut(col_no).unwrap();
                    stack.push_front(*ch as char);
                })
        });

    stacks
}

fn run_instructions(mut stacks:Vec<VecDeque<char>>, instructions:Vec<(usize,usize,usize)>, preserve_order:bool) -> String {
    for &(num, from, to) in instructions.iter() {
        let mut items = (0..num).into_iter()
            .map(|_| stacks.get_mut(from).unwrap().pop_back().unwrap())
            .collect::<Vec<_>>();

        if preserve_order {
            items.reverse();
        }

        items.into_iter()
            .for_each(|item| stacks.get_mut(to).unwrap().push_back(item));
    }

    stacks.iter().filter(|stack| !stack.is_empty())
        .map(|stack| stack.back().unwrap())
        .collect::<String>()
}

fn part1(lines:Vec<&str>) -> String {
    run_instructions(parse_map(&lines), parse_moves(&lines), false)
}


fn part2(lines:Vec<&str>) -> String {
    run_instructions(parse_map(&lines), parse_moves(&lines), true)
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!("CMZ", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_05.txt");

        assert_eq!("FJSRQCFTN", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!("MCD", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_05.txt");

        assert_eq!("CJVLJQPHS", solve(input.to_string(), Part2));
    }
}
