use std::collections::VecDeque;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let monkeys = input.split("\n\n").into_iter()
        .map(|monkey_str| monkey_str.lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    match part {
        Part::Part1 => part1(monkeys),
        Part::Part2 => part2(monkeys)
    }
}

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn do_op(&self, old:u64) -> u64 {
        match &self {
            Operation::Add(value) => old + value,
            Operation::Multiply(value) => old * value,
            Operation::Square => old * old,
        }
    }
}

struct Monkey {
    items:VecDeque<u64>,
    operation:Operation,
    test_divisor:u64,
    to_monkey_index_if_true:usize,
    to_monkey_index_if_false:usize,
    inspect_count:usize,
}

impl Monkey {
    fn new(input:&Vec<&str>) -> Monkey {

        let items = input[1].to_string()[16..].split([' ',','])
            .into_iter().filter(|item| item.len() > 0)
            .map(|item| item.parse::<u64>().unwrap())
            .collect::<VecDeque<_>>();

        let operation_str = input[2].to_string()[17..].to_string().split(' ')
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let test = input[3].split(' ').last().unwrap().parse::<u64>().unwrap();
        let next_monkey1 = input[4].split(' ').last().unwrap().parse::<usize>().unwrap();
        let next_monkey2 = input[5].split(' ').last().unwrap().parse::<usize>().unwrap();

        let operation = if operation_str[0].eq("old") && operation_str[2].eq("old") {
            Operation::Square
        } else if operation_str[1].eq("*") {
            Operation::Multiply(operation_str[2].parse::<u64>().unwrap())
        } else {
            Operation::Add(operation_str[2].parse::<u64>().unwrap())
        };

        Monkey{items, test_divisor: test, to_monkey_index_if_true: next_monkey1, to_monkey_index_if_false: next_monkey2, inspect_count:0, operation}
    }

    fn do_turn(&mut self, part_two:bool, divisor:u64) -> Vec<(usize,u64)> {
        let mut output = vec![];
        self.inspect_count += self.items.len();

        while self.items.len() > 0 {
            let item = self.items.pop_front().unwrap();
            let mut next_value = self.operation.do_op(item);
            if !part_two {
                next_value = next_value / 3;
            } else {
                next_value = next_value % divisor;
            }

            if next_value % self.test_divisor == 0 {
                output.push((self.to_monkey_index_if_true, next_value));
            } else {
                output.push((self.to_monkey_index_if_false, next_value));
            }
        }

        output
    }
}

fn run_monkey_business(input: Vec<Vec<&str>>, rounds:usize, part2:bool) -> String {
    let mut monkeys = input.iter().map(|item| Monkey::new(item)).collect::<Vec<_>>();
    let divisor:u64 = monkeys.iter().fold(1, |acc, m  | acc * m.test_divisor);

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let next_items = monkeys[i].do_turn(part2, divisor);
            for (next_index, next_item) in next_items {
                monkeys[next_index].items.push_back(next_item);
            }
        }
    }

    monkeys.sort_by( |a,b| b.inspect_count.cmp(&a.inspect_count) );
    (monkeys[0].inspect_count * monkeys[1].inspect_count).to_string()
}

fn part1(input : Vec<Vec<&str>>) -> String {
    run_monkey_business(input, 20, false)
}

fn part2(input : Vec<Vec<&str>>) -> String {
    run_monkey_business(input, 10_000, true)
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    #[test]
    fn test1() {

        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";


        assert_eq!("10605", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_11.txt");

        assert_eq!("54752", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        assert_eq!("2713310158", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_11.txt");

        assert_eq!("13606755504", solve(input.to_string(), Part2));
    }


    #[test]
    fn test_failing_data() {
        let input = include_str!("../../input/input_11_failing.txt");

        assert_eq!("55216", solve(input.to_string(), Part1));
        assert_eq!("12848882750", solve(input.to_string(), Part2));
    }



}
