use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Node {
    name:String,
    rate:i32,
    neighbours:Vec<String>
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node[name:{}, rate:{}, edges:{:?}",self.name, self.rate, self.neighbours)
    }
}

impl Node {
    fn parse(line:&str) -> Node {
        let columns = line.split(['=',';',',',' '])
            .filter(|col| col.len() > 0)
            .collect::<Vec<_>>();

        let name = columns[1].to_string();
        let rate = columns[5].parse::<i32>().unwrap();
        let mut neighbours = vec![];
        for n in 10..columns.len() {
            neighbours.push(columns[n].to_string());
        }

        Node{name, rate, neighbours}
    }

}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct State {
    pos:String,
    pos_helper:String,
    opened:Vec<String>,
    remaining:Vec<String>,
    volume:i32,
    flow:i32,
    turn_left:i32,
    max_theoretical:i32,
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(&other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.turn_left.cmp(&other.turn_left);
        if order.eq(&Ordering::Equal) {
            return other.max_theoretical.cmp(&self.max_theoretical);
        } else {
            order
        }
    }
}

impl State {
    fn first_state(graph:&Graph, node:&Node, no_turns:i32) -> State {
        let opened:Vec<String> = vec![];
        let pos = node.name.clone();
        let pos_helper = node.name.clone();
        let mut remaining_nodes = graph.nodes.iter().filter(|n| n.rate > 0).collect::<Vec<_>>();
        let flow = 0;
        let volume = 0;
        remaining_nodes.sort_by(|a,b| b.rate.cmp(&a.rate));
        let remaining = remaining_nodes.iter().map(|n| n.name.clone()).collect();
        let mut state = State{pos, pos_helper, opened, remaining, turn_left:no_turns, volume, flow, max_theoretical:0};
        state.max_theoretical = state.calc_theoretical_max_volume(graph);
        state
    }

    fn next_state(&self, graph:&Graph, next_node:&Node, next_node_helper:&Node, distance:i32) -> State {
        let pos = next_node.name.clone();
        let pos_helper = next_node_helper.name.clone();
        let mut opened:Vec<String> = self.opened.clone();
        let remaining = self.remaining.iter().filter(|&n|pos_helper.ne(n) && pos.ne(n)).map(|s|s.clone()).collect();
        let flow = self.flow + next_node.rate;
        let volume = self.volume + distance * self.flow;
        let turn_left = self.turn_left - distance;

        opened.push(next_node.name.clone());

        let mut next_state = State{pos, pos_helper, opened, remaining, flow, volume, turn_left, max_theoretical:0};
        next_state.max_theoretical = next_state.calc_theoretical_max_volume(graph);
        next_state
    }

    fn calc_theoretical_max_volume(&self, graph:&Graph) -> i32 {
        let turns_left =self.turn_left;
        let mut max_volume = self.volume;
        let mut current_flow = self.flow;
        let mut remaining_index = 0;
        for t in 1..=turns_left {
            max_volume += current_flow;
            if t % 2 == 0 {
                if remaining_index != self.remaining.len() {
                    let node = graph.get_node(self.remaining[remaining_index].as_str());
                    current_flow += node.rate;
                    remaining_index += 1;
                }
            }

        }
        max_volume
    }

    fn calc_end_flow(&self) -> i32 {
        let mut total_volume = self.volume;
        for _ in 1..=self.turn_left {
            total_volume += self.flow;
        }
        total_volume
    }

    fn is_done(&self) -> bool {
        self.remaining.is_empty()
    }
}

struct Graph {
    nodes:Vec<Node>,
}

impl Graph {
    fn parse(input: String) -> Graph {
        let nodes = input.lines().map(|line| Node::parse(line)).collect::<Vec<_>>();
        Graph { nodes }
    }

    fn get_node(&self, node_name: &str) -> &Node {
        self.nodes.iter().find(|node| node.name.as_str().eq(node_name)).unwrap()
    }

    fn bfs(&self, state:&State) -> Vec<(i32, String)> {
        let opened_valves = &state.opened;
        let origin_node = self.get_node(state.pos.as_str());
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = vec![];

        // Insert origin node as visited
        visited.insert(origin_node);
        queue.push_back((0, origin_node));

        while !queue.is_empty() {
            let (distance, current_node) = queue.pop_front().unwrap();

            // Is this a node we can open?
            if !opened_valves.contains(&current_node.name) && current_node.ne(origin_node) && current_node.rate > 0 {
                // Add an extra distance since it costs to open the valve
                let total_cost = distance + 1;
                if total_cost > state.turn_left {
                    continue;
                }
                result.push((distance + 1, current_node.name.clone()));
            }

            // Check neighbours
            current_node.neighbours.iter()
                .map(|name| self.nodes.iter().find(|node| node.name.eq(name)).unwrap())
                .for_each(|next_node |{
                    // insert as visited
                    if !visited.contains(&next_node) {
                        visited.insert(next_node);
                        // add to queue
                        queue.push_back((distance + 1, next_node));
                    }
                })
        }

        result
    }

    fn solve(&self, no_turns:i32) -> i32 {
        let start_node = self.get_node("AA");
        let start_state = State::first_state(self, start_node, no_turns);
        let mut queue = BinaryHeap::new();
        let mut results = vec![];
        let mut iterations = 0;

        // Add first state
        queue.push(Reverse(start_state));

        while !queue.is_empty() {
            let current_state = queue.pop().unwrap().0;
            iterations += 1;

            if current_state.is_done() {
                results.push((current_state.calc_end_flow(), current_state));
                // Order by best result
                results.sort_by(|(end_flow_a, _),(end_flow_b, _)| end_flow_b.cmp(end_flow_a));
                continue;
            } else if !results.is_empty() && results.first().unwrap().0 >= current_state.max_theoretical {
                // Already evaluated better option
                continue;
            } else {

                let next_nodes = self.bfs(&current_state);

                if next_nodes.is_empty() {
                    results.push((current_state.calc_end_flow(), current_state));
                    // Order by best result
                    results.sort_by(|(end_flow_a, _),(end_flow_b, _)| end_flow_b.cmp(end_flow_a));
                    continue;
                }

                // Perform BFS and find next steps
                for (next_dist, next_node_name) in next_nodes {
                    let next_node = self.get_node(next_node_name.as_str());
                    let next_state = current_state.next_state(self, next_node, next_node, next_dist);
                    queue.push(Reverse( next_state));
                }
            }
        }
        println!("iterations:{}", iterations);
        results.first().unwrap().0
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        self.nodes.iter().for_each(|node| {
            let s = format!("{:?}, ", node);
            str.push_str(s.as_str());
        });

        write!(f,"{}", str)
    }
}

fn part1(input : String) -> String {
    let graph = Graph::parse(input);
    let results = graph.solve(30);
    results.to_string()
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
        assert_eq!("1651", solve(TEST_INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_16.txt");
        assert_eq!("1775", solve(input.to_string(), Part1));
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
