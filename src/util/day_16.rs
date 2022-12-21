use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

struct State {
    pos:String,
    visited:Vec<String>,
    flow_rate:u32,
    volume:u32,
    minute:u32,
    done:bool,
}

impl State {
    fn new(pos:&str, visited:Vec<String>, flow_rate:u32, volume:u32, minute:u32) -> State {
        State{pos:pos.to_string(), visited, flow_rate, volume, minute, done:false}
    }

    fn from(other:&State) -> State {
        State{pos:other.pos.to_string(),visited:other.visited.clone(), flow_rate:other.flow_rate, volume:other.volume,
            minute:other.minute, done:other.done}
    }

    fn fast_forward(&mut self) {
        loop {

            if self.minute < 30 {
                // Add flow
                self.volume += self.flow_rate;
                self.minute += 1;
            } else {
                break;
            }
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "State[pos:{}, minute:{}, visited:{:?}, rate:{}, volume:{}]",self.pos, self.minute, self.visited, self.flow_rate, self.volume)
    }
}

struct Graph {
    nodes:Vec<Node>,
    max_flow:u32,
}

impl Graph {
    fn parse(input:String) -> Graph {
        let nodes = input.lines().map(|line| Node::parse(line)).collect::<Vec<_>>();
        let max_flow = nodes.iter().map(|node| node.rate).sum::<u32>();
        Graph{nodes, max_flow}
    }

    fn get_node(&self, node_name:&str) -> &Node {
        self.nodes.iter().find(|node| node.name.as_str().eq(node_name)).unwrap()
    }

    fn get_next_states(&self, state:&State)  -> Vec<State> {
        let destinations = self.bfs(state.pos.as_str(), &state.visited);
        //println!("dest:{:?}", destinations);
        let mut next_states = vec![];

        if destinations.is_empty() {
            let mut next_state = State::from(state);
            //println!("--> no destinations:{:?}", state);
            next_state.done = true;
            next_states.push(next_state);
        } else {
            for (dist, dest_node) in destinations {
                let next_volume = state.flow_rate * (dist + 1) + state.volume;
                let next_flow_rate = state.flow_rate + dest_node.rate;
                let next_minute = state.minute + dist + 1; // 1 for opening
                let mut next_visited = state.visited.iter().map(|s| s.to_string()).collect::<Vec<_>>();
                next_visited.push(dest_node.name.clone());

                if next_minute < 30 {
                    let next_state = State::new(dest_node.name.as_str(), next_visited, next_flow_rate, next_volume, next_minute);
                    next_states.push(next_state);
                } else {
                    let mut next_state = State::from(state);
                    //println!("--> no destinations:{:?}", state);
                    next_state.done = true;
                    next_states.push(next_state);
                }
            }
        }

        next_states
    }

    fn bfs(&self, origin_node_node:&str, opened_valves:&Vec<String>) -> Vec<(u32, &Node)> {
        let origin_node = self.get_node(origin_node_node);
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = vec![];

        // Insert as visited
        visited.insert(&origin_node.name);
        queue.push_back((0, origin_node));

        while !queue.is_empty() {
            let (current_dist, node) = queue.pop_front().unwrap();

            if !opened_valves.contains(&node.name) && origin_node.ne(node) && node.rate > 0 {
                // Found possible destination
                result.push((current_dist, node));
            }

            // Explore edges
            for adjacent_name in node.neighbours.iter() {
                let adjacent_node = self.get_node(adjacent_name.as_str());
                if !visited.contains(&adjacent_name) {
                    // Unexplored node
                    visited.insert(adjacent_name);
                    queue.push_back((current_dist + 1, adjacent_node));
                }
            }
        }

        result
    }


    fn solve(&mut self) -> u32 {
        let mut results = vec![];
        let start_state = State::new("AA", vec![], 0, 0, 0);
        let mut prev_result = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(start_state);

        while !queue.is_empty() {
            let mut state = queue.pop_front().unwrap();

            //println!("Process state:{:?}", state);

            if state.flow_rate == self.max_flow || state.done {
                state.fast_forward();
                let minute = state.minute;

                if let Some(prev_vol) = prev_result.get_mut(&minute) {
                    if *prev_vol >= state.volume {
                        // Already found solution
                    } else {

                        // Done
                        //println!("---> {:?}", state);

                        state.fast_forward();

                        *prev_vol = state.volume;

                        println!("---> New better solution at {} -> {}", minute, state.volume);
                        results.push(state);
                    }
                } else {
                    println!(">>>>> New better solution at {} -> {}", minute, state.volume);
                    prev_result.insert(minute, state.volume);
                    results.push(state);
                }
                continue;
            } else {
                // Check if we already have solution
                if let Some(prev_vol) = prev_result.get_mut(&state.minute) {
                    if *prev_vol > state.volume {
                        // No point evaluating this...
                        continue;
                    }
                }
            }

            // Check options for state
            let destination_states = self.get_next_states(&state);

            for next_state in destination_states {
                queue.push_back(next_state);
            }

        }

        results.sort_by(|a,b| a.volume.cmp(&b.volume));
        results.reverse();
        println!("Best:{:?}", results.first().unwrap());
        results.first().unwrap().volume
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

#[derive(Eq, PartialEq)]
struct Node {
    name:String,
    rate:u32,
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
        let rate = columns[5].parse::<u32>().unwrap();
        let mut neighbours = vec![];
        for n in 10..columns.len() {
            neighbours.push(columns[n].to_string());
        }

        Node{name, rate, neighbours}
    }

}

fn part1(input : String) -> String {

    let mut graph = Graph::parse(input);
    //println!("{:?}", graph);

    graph.solve().to_string()
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

    //#[test]
    fn _test_part1() {
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
