use std::cmp::Ordering;
use std::collections::VecDeque;
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

trait Packet {
    fn len(&self) -> usize;
    fn is_integer(&self) -> bool;
    fn get_integer_value(&self) -> i32;
    fn get_element(&self, index:usize) -> &dyn Packet;
    fn compare(&self, other:&dyn Packet) -> Ordering;
}

fn parse(line:&str) -> Box<dyn Packet> {
    let mut chars = line.chars().collect::<VecDeque<char>>();
    let mut stack = VecDeque::new();
    while !chars.is_empty() {
        let ch = chars.pop_front().unwrap();
        if ch == '[' {
            // Start of list
            stack.push_front(List::new());
        } else if ch == ']' {
            if stack.len() == 1 {
                // Final element
                break;
            }
            // Pop list on top of stack and add it to parent
            let list = stack.pop_front().unwrap();
            if !stack.is_empty() {
                stack.front_mut().unwrap().add(Box::new(list));
            }
        } else if ch.is_digit(10) {
            let mut number_str = String::from(ch);
            while chars.front().unwrap().is_digit(10) {
                number_str.push(chars.pop_front().unwrap());
            }

            let integer = Integer::new(number_str.parse().unwrap());
            stack.front_mut().unwrap().add(Box::new(integer));

        }
    }

    Box::new(stack.pop_front().unwrap())
}

struct Integer {
    number:i32,
}

impl Integer {
    fn new(number:i32) -> Integer {
        Integer{number}
    }
}

struct List {
    list:Vec<Box<dyn Packet>>,
}


impl Packet for Integer {

    fn len(&self) -> usize {
        0
    }
    fn is_integer(&self) -> bool {
        true
    }
    fn get_integer_value(&self) -> i32 {
        self.number
    }
    fn get_element(&self, _index:usize) -> &dyn Packet {
        panic!("...");
    }

    fn compare(&self, other: &dyn Packet) -> Ordering {
        if other.is_integer() {
            // Integer to integer comparison
            self.number.cmp(&other.get_integer_value())
        } else {
            List::from_number(self.number).compare(other)
        }
    }
}

impl Packet for List {

    fn len(&self) -> usize {
        self.list.len()
    }
    fn is_integer(&self) -> bool {
        false
    }
    fn get_integer_value(&self) -> i32 {
        panic!("not supported");
    }
    fn get_element(&self, index:usize) -> &dyn Packet {
        &**self.list.get(index).unwrap()
    }

    fn compare(&self, other: &dyn Packet) -> Ordering {
        if other.is_integer() {
            self.compare(&List::from_number(other.get_integer_value()))
        } else {

            // Check if of the lists are empty any is empty
            if self.len() == 0 && other.len() == 0 {
                return Ordering::Equal;
            } else if self.len() == 0 && other.len() > 0 {
                return Ordering::Less;
            } else if self.len() > 0 && other.len() == 0 {
                return Ordering::Greater;
            }

            let mut index = 0;
            while index < self.len() {
                let next_element = self.get_element(index);

                if other.len() > index {
                    let res = next_element.compare(other.get_element(index));
                    if res != Ordering::Equal {
                        return res;
                    }
                } else {
                    // Other list ran out of items
                    return Ordering::Greater;
                }

                // Advance
                index += 1;
            }

            if self.len() == other.len() {
                return Ordering::Equal;
            } else {
                return Ordering::Less;
            }
        }
    }
}

impl List {
    fn new() -> List {
        List{list:vec![]}
    }

    fn from_number(integer:i32) -> List {
        List{list:vec![Box::new(Integer::new(integer))]}
    }

    fn add(&mut self, element: Box<dyn Packet>) {
        self.list.push(element);
    }
}



fn part1(input : String) -> String {
    input.split("\n\n").enumerate().map(|(no,two_lines)| {
        let mut it = two_lines.lines().into_iter();
        let left = parse(it.next().unwrap());
        let right = parse(it.next().unwrap());

        (no + 1, left.compare(&*right) == Ordering::Less)
    })
        .filter(|(_,right_order)| *right_order)
        .map(|(index,_)| index)
        .sum::<usize>().to_string()
}

fn part2(input : String) -> String {
    let mut packets = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse(line))
        .collect::<Vec<_>>();

    // Push divider packets
    packets.push(parse("[[2]]"));
    packets.push(parse("[[6]]"));

    // Sort packets
    packets.sort_by( |a,b| a.compare(b.as_ref()));

    // Find index of divider packets
    let divider_packet1 = parse("[[2]]");
    let divider_packet2 = parse("[[6]]");

    packets.iter().enumerate()
        .filter(|(_, element)|
            divider_packet1.compare(element.as_ref()) == Ordering::Equal || divider_packet2.compare(element.as_ref()) == Ordering::Equal)
        .map(|(index,_)| index+1)
        .product::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!("13", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_13.txt");

        assert_eq!("5350", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!("140", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_13.txt");

        assert_eq!("19570", solve(input.to_string(), Part2));
    }

}
