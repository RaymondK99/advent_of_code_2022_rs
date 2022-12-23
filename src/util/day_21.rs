use std::collections::VecDeque;
use util::day_21::Side::{Left, Right};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}
#[derive(Debug, Copy, Clone)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
struct Monkey {
    name:String,
    operation:Operation,
}

#[derive(Debug,  Clone)]
enum Operation {
    Number(i64),
    Binary(Operand, String, String),
}

#[derive(Debug,  Copy, Clone)]
enum Operand {
    Plus,
    Minus,
    Mult,
    Div,
}

fn parse(line:&str) -> Monkey {
    let fields = line.split([':',' '])
        .filter(|field| field.len() > 0)
        .collect::<Vec<_>>();

    if fields.len() == 2 {
        let number = fields[1].parse::<i64>().unwrap();
        let operation = Operation::Number(number);
        Monkey{name:fields[0].to_string(),operation}
    } else {
        let name = fields[0].to_string();
        let left = fields[1].to_string();
        let op = fields[2];
        let right = fields[3].to_string();

        let operand = match op {
            "+" => Operand::Plus,
            "-" => Operand::Minus,
            "*" => Operand::Mult,
            "/" => Operand::Div,
            &_ => {
                panic!("..");
            }
        };
        let operation = Operation::Binary(operand, left, right);
        Monkey{name, operation}
    }
}

fn contains_monkey(current:&String, search:&str, monkeys:&Vec<Monkey>) -> bool {
    let monkey = monkeys.iter().find(|monkey| monkey.name.eq(current)).unwrap();
    if monkey.name.as_str().eq(search) {
        return true;
    }

    let operation = &monkey.operation;

    match operation {
        Operation::Number(_) => false,
        Operation::Binary(_, left, right) => contains_monkey(left, search, monkeys) || contains_monkey(right, search, monkeys),
    }
}

fn find_path(current:&str, search:&str, monkeys:&Vec<Monkey>, path:Vec<(Side, Operand, i64)>) -> Vec<(Side, Operand, i64)> {
    let monkey = monkeys.iter().find(|monkey| monkey.name.eq(current)).unwrap();

    if monkey.name.as_str().eq(search) {
        // Found leaf node
        return path;
    }

    match &monkey.operation {
        Operation::Number(_) => panic!("...."),
        Operation::Binary(operand, left, right) => {
            let mut next_path = path.clone();
            // Check which side contains the wanted monkey
            if contains_monkey(left, search, monkeys) {
                let right_value = resolve(right, monkeys);
                next_path.push((Right, *operand, right_value));
                find_path(left, search, monkeys, next_path)
            } else {
                let left_value = resolve(left, monkeys);
                next_path.push((Left, *operand, left_value));
                find_path(right, search, monkeys, next_path)
            }
        }
    }
}

fn resolve(monkey_name:&str, monkeys:&Vec<Monkey>) -> i64 {
    let monkey = monkeys.iter().find(|m| m.name.as_str().eq(monkey_name)).unwrap();

    match &monkey.operation {
        Operation::Number(number) => *number,
        Operation::Binary(operand, left, right) => {
            let left_value = resolve(left.as_str(), monkeys);
            let right_value = resolve(right.as_str(), monkeys);

            match operand {
                Operand::Plus => left_value + right_value,
                Operand::Minus => left_value - right_value,
                Operand::Mult => left_value * right_value,
                Operand::Div => left_value / right_value,
            }
        }
    }
}


fn part1(input : String) -> String {
    let monkeys = input.lines().map(|line| parse(line)).collect::<Vec<_>>();
    resolve("root", &monkeys).to_string()
}

fn part2(input : String) -> String {
    let monkeys = input.lines().map(|line| parse(line)).collect::<Vec<_>>();
    let mut path = find_path("root", "humn", &monkeys, vec![]).iter().copied().collect::<VecDeque<_>>();

    println!("path:{:?}", path);

    let (_,_,mut humn_value) = path.pop_front().unwrap();
    while !path.is_empty() {
        let (side, operand, value) = path.pop_front().unwrap();
        match operand {
            Operand::Plus => {
                match side {
                    _ =>  humn_value = humn_value - value,
                    //Left =>  humn_value = value - humn_value,
                }
            },
            Operand::Minus => humn_value += value,
            Operand::Mult => {
                match side {
                    _ =>  humn_value = humn_value / value,
                    //Left =>  humn_value = value / humn_value,
                }
            },
            Operand::Div => humn_value *= value,
        }
    }

    humn_value.to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const TEST_INPUT:&str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test1() {
        assert_eq!("152", solve(TEST_INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_21.txt");
        assert_eq!("85616733059734", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("301", solve(TEST_INPUT.to_string(), Part2));
    }

    //#[test]
    fn _test_part2() {
        // TODO: FIX
        // Too high:7_243_227_128_687
        let input = include_str!("../../input/input_21.txt");
        assert_eq!("1", solve(input.to_string(), Part2));
    }
}
