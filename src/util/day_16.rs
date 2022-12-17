use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}


#[derive(Debug, Eq, PartialEq)]
struct Valve {
    name:String,
    rate:u32,
}


struct Node {
    //valve:Valve,
    //neighbours:Vec<String>
}


impl Node {
    fn parse(line:&str) {
        let columns = line.split(['=',';',',',' '])
            .filter(|col| col.len() > 0)
            .collect::<Vec<_>>();

        let name = columns[1];
        let rate = columns[5].parse::<u32>().unwrap();
        let mut neighbors = vec![];
        for n in 10..columns.len() {
            neighbors.push(columns[n]);
        }

        println!("{},{},{:?}", name, rate, neighbors);
    }

}

fn part1(input : String) -> String {

    input.lines().for_each(|line| Node::parse(line));
    "1".to_string()
}

fn part2(_input : String) -> String {
    "1".to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const TEST_INPUT:&str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test1() {
        assert_eq!("1", solve(TEST_INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_16.txt");

        assert_eq!("1", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("1", solve(TEST_INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_16.txt");

        assert_eq!("1", solve(input.to_string(), Part2));
    }
}
