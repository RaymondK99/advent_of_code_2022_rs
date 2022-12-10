use std::collections::VecDeque;
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}


struct Device {
    current_cycle:i32,
    register:i32,
    instructions:VecDeque<Box<dyn Instruction>>,
    signals:Vec<i32>,
    display:Vec<char>,
}

impl Device {

    fn new(input:String) -> Device {
        Device{ current_cycle:1, register:1, instructions:Self::parse_instructions(input), signals: vec![], display: vec![] }
    }

    fn parse(line:&str) -> Box<dyn Instruction> {
        let instruction = line.split(' ').collect::<Vec<_>>();
        return if instruction[0].eq("noop") {
            Box::new(Noop {})
        } else {
            Box::new(Addx { argument: instruction[1].parse().unwrap(), cycles: 0 })
        }
    }

    fn parse_instructions(input:String) -> VecDeque<Box<dyn Instruction>> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn run_instructions(&mut self) {

        while !self.instructions.is_empty() {
            let mut instruction = self.instructions.pop_front().unwrap();

            // Build signals
            if (self.current_cycle - 20) % 40 == 0 {
                self.signals.push(self.current_cycle * self.register);
            }

            // Build display
            let crt = (self.current_cycle - 1) % 40;
            if self.register.abs_diff(crt) <= 1 {
                self.display.push('*');
            } else {
                self.display.push('.');
            };

            if self.current_cycle % 40 == 0 {
                self.display.push('\n');
            }

            // Process instruction
            if !instruction.process(&mut self.register) {
                self.instructions.push_front(instruction);
            }

            // Increment cycle
            self.current_cycle += 1;
        }
    }
}

trait Instruction {
    fn process(&mut self, register:&mut i32) -> bool;
}

#[derive(Debug)]
struct Noop {}

#[derive(Debug)]
struct Addx {
    argument:i32,
    cycles:u8,
}

impl Instruction for Addx {
    fn process(&mut self, register:&mut i32) -> bool {
        if self.cycles == 0 {
            self.cycles += 1;
            false
        } else {
            *register += self.argument;
            true
        }
    }
}

impl Instruction for Noop {
    fn process(&mut self, _register:&mut i32) -> bool {
        true
    }
}


fn part1(input : String) -> String {
    let mut device = Device::new(input);
    device.run_instructions();
    device.signals.iter().sum::<i32>().to_string()
}

fn part2(input : String) -> String {
    let mut device = Device::new(input);
    device.run_instructions();
    device.display.iter().collect()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        assert_eq!("13140", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_10.txt");

        assert_eq!("14780", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {


        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!("**..**..**..**..**..**..**..**..**..**..
***...***...***...***...***...***...***.
****....****....****....****....****....
*****.....*****.....*****.....*****.....
******......******......******......****
*******.......*******.......*******.....
", solve(input.to_string(), Part2));

    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_10.txt");

        // ELPLZGZL
        assert_eq!("****.*....***..*....****..**..****.*....
*....*....*..*.*.......*.*..*....*.*....
***..*....*..*.*......*..*......*..*....
*....*....***..*.....*...*.**..*...*....
*....*....*....*....*....*..*.*....*....
****.****.*....****.****..***.****.****.
", solve(input.to_string(), Part2));
    }
}
