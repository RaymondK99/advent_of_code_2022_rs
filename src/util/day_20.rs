use std::collections::VecDeque;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

fn parse(input:String, key:i64) -> VecDeque<(usize,i64)>{
    input.lines()
        .map(|line| key * line.parse::<i64>().unwrap())
        .enumerate().collect()
}

fn forward(mut n:i64, numbers:&mut VecDeque<(usize,i64)>) {
    n = n % numbers.len() as i64;
    numbers.rotate_left(n as usize);
}

fn reverse(mut n:i64, numbers:&mut VecDeque<(usize,i64)>) {
    n = n % numbers.len() as i64;
    numbers.rotate_right(n as usize);
}

fn mix(times:usize, numbers:&mut VecDeque<(usize, i64)>) -> i64 {

    for _ in 0..times {
        for n in 0..numbers.len() {
            mix_seq_no(n, numbers);
        }
    }

    let index_of_zero = get_index(0, &numbers);
    let index_1000th = (index_of_zero + 1000) % numbers.len();
    let index_2000th = (index_of_zero + 2000) % numbers.len();
    let index_3000th = (index_of_zero + 3000) % numbers.len();

    [index_1000th, index_2000th, index_3000th].iter()
        .map(|index| numbers.get(*index).unwrap())
        .map(|(_,number)| *number)
        .sum()
}

fn mix_seq_no(step_no:usize, numbers:&mut VecDeque<(usize, i64)>) {
    // Find right sequence
    while step_no != numbers.front().unwrap().0 {
        forward(1, numbers);
    }


    let (seq_no, number) = numbers.pop_front().unwrap();

    if number < 0 {
        reverse(number.abs(), numbers);
    } else {
        forward(number.abs(), numbers);
    }

    numbers.push_front((seq_no, number));
}

fn get_index(number:i64, numbers:&VecDeque<(usize,i64)>) -> usize {
    numbers.iter().enumerate().find(|(_,(_, num))|  *num == number).unwrap().0
}

fn part1(input : String) -> String {
    let mut numbers = parse(input, 1);
    mix(1, &mut numbers).to_string()
}

fn part2(input : String) -> String {
    let mut numbers = parse(input, 811589153);
    mix(10, &mut numbers).to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "1
2
-3
3
-2
0
4";

        assert_eq!("3", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_20.txt");

        assert_eq!("3346", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "1
2
-3
3
-2
0
4";

        assert_eq!("1623178306", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_20.txt");
        assert_eq!("4265712588168", solve(input.to_string(), Part2));
    }
}
