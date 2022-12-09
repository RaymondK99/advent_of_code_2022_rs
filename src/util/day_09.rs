use std::collections::{HashSet, VecDeque};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Pos {
    x:i32,
    y:i32,
}

#[derive(Debug)]
struct Rope {
    knots:VecDeque<Pos>,
}

impl Pos {
    fn initial() -> Pos {
        Pos{x:0,y:0}
    }

    fn add_delta(&self, delta:(i32,i32)) -> Pos {
        Pos{x:self.x + delta.0, y:self.y + delta.1}
    }

    fn get_dist(&self,other:&Pos) -> u32 {
        let horizontal_diff = self.x.abs_diff(other.x);
        let vertical_diff = self.y.abs_diff(other.y);

        return if horizontal_diff == 1 && vertical_diff == 1 {
            1
        } else {
            horizontal_diff + vertical_diff
        }
    }
}

impl Rope {

    fn new(no_tails:usize) -> Rope {
        Rope{knots:(0..no_tails).into_iter().map(|_| Pos::initial()).collect()}
    }

    fn process_moves(&mut self, input:String) -> Vec<Pos> {
        let mut moves = vec![Pos::initial()];
        input.lines()
            .for_each(|line| moves.append(&mut self.process_move(line) ));
        moves
    }

    fn process_move(&mut self, line:&str) -> Vec<Pos> {
        let mut it = line.split(' ').into_iter();
        let command = it.next().unwrap().as_bytes()[0] as char;
        let moves = it.next().unwrap().parse::<u32>().unwrap();
        let mut tail_moves = vec![];

        for _ in 0..moves {

            // Move tail
            for tail_index in 0..self.knots.len() {

                // Pop first knot
                let mut knot = self.knots.pop_front().unwrap();

                if tail_index == 0 {
                    // This is the head, move according to command
                    Self::move_head(&mut knot, command);
                } else {
                    // Fetch head
                    let head = self.knots.back().unwrap();

                    if let Some(tail_delta) = Self::get_tail_delta(head, &knot) {
                        knot = knot.add_delta(tail_delta);

                        if tail_index == self.knots.len() {
                            // Is last tail?
                            tail_moves.push(knot.clone());
                        }
                    }
                }

                self.knots.push_back(knot);
            }
        }

        tail_moves
    }

    fn move_head(head: &mut Pos, dir:char) {
        let delta_pos = match dir {
            'R' => (1,0),
            'L' => (-1,0),
            'U' => (0,-1),
            'D' => (0,1),
            _ => panic!(""),
        };

        // move header
        *head = head.add_delta(delta_pos);
    }

    fn get_tail_delta(head:&Pos, tail:&Pos) -> Option<(i32, i32)> {
        if head.get_dist(tail) > 1 {
            // Only move tail if distance is greater than 1
            let deltas = if head.x == tail.x || head.y == tail.y {
                [(1,0),(-1,0),(0,1),(0,-1)]
            } else {
                [(-1,-1),(-1,1),(1,-1),(1,1)]
            };

            let delta = deltas.iter().find( |&delta| tail.add_delta(*delta).get_dist(head) < 2 ).unwrap();
            Some(*delta)
        } else {
            None
        }
    }
}

fn part1(input : String) -> String {
    let mut rope = Rope::new(2);
    let tail_moves = rope.process_moves(input).iter().copied().collect::<HashSet<Pos>>();
    tail_moves.len().to_string()
}


fn part2(input : String) -> String {
    let mut rope = Rope::new(10);
    let tail_moves = rope.process_moves(input).iter().copied().collect::<HashSet<Pos>>();
    tail_moves.len().to_string()}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!("13", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_09.txt");

        assert_eq!("6384", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!("36", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_09.txt");

        assert_eq!("2734", solve(input.to_string(), Part2));
    }
}
