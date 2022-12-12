use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

#[derive(Eq, Debug, PartialEq, Copy, Clone, Hash, PartialOrd, Ord)]
struct Pos {
    x:usize,
    y:usize,
}

impl Pos {
    fn new(x:usize,y:usize) -> Pos {
        Pos{x,y}
    }
}

struct Map {
    map:Vec<Vec<char>>,
    height:usize,
    width:usize,
}

impl Map {
    fn new(input:String) -> Map {
        let map = input.lines()
            .map(|line| line.chars()
                .collect())
            .collect::<Vec<Vec<char>>>();

        let height = map.len();
        let width = map.first().unwrap().len();

        Map{map, height, width}
    }

    fn get_height(&self, pos:&Pos) -> u8 {
        let height = *self.map.get(pos.y).unwrap().get(pos.x).unwrap();
        if height == 'S' {
            'a' as u8
        } else if height == 'E' {
            'z' as u8
        } else {
            height as u8
        }
    }


    fn is_end(&self, pos:&Pos) -> bool {
        *self.map.get(pos.y).unwrap().get(pos.x).unwrap() as char == 'E'
    }

    fn get_moves(&self, current:&Pos) -> Vec<Pos> {
        let current_height = self.get_height(&current);
        let mut moves = vec![];

        if current.x > 0 {
            moves.push(Pos::new(current.x-1,current.y));
        }
        if current.x < self.width - 1 {
            moves.push(Pos::new(current.x+1,current.y));
        }
        if current.y > 0 {
            moves.push(Pos::new(current.x,current.y-1));
        }
        if current.y < self.height - 1 {
            moves.push(Pos::new(current.x,current.y+1));
        }

        moves.iter()
            .map(|p| (p, self.get_height(p)))
            .filter(|(_, dest_height)| *dest_height <= current_height || *dest_height == current_height + 1)
            .map(|(p,_)| *p)
            .collect()
    }


    fn invert(&mut self) -> &mut Map {
        self.map.iter_mut()
            .for_each(| row| row.iter_mut()
                .for_each(|ch| {
                    let next_char = match *ch {
                        'E' => 'S',
                        'S' => 'a',
                        'a' => 'E',
                        _ => ('z' as u8 - (*ch as u8 - 'a' as u8)) as char
                    };
                    *ch = next_char;
                }));
        self
    }

    fn find_end(&self) -> Option<usize> {
        let start = self.map.iter().enumerate()
            .flat_map(|(y,row)| row.iter().enumerate()
                .map(move |(x,ch)| (Pos::new(x,y),ch)))
            .find(|(_, &ch)| ch == 'S').unwrap().0;

        let mut visited:HashMap<Pos, usize> = HashMap::new();
        let mut queue = BinaryHeap::new();

        // Add starting position
        queue.push(Reverse((0, start)));

        while !queue.is_empty() {
            let (current_distance, current) = queue.pop().unwrap().0;
            let prev_distance_opt = visited.get(&current);

            // Did we reach the end
            if self.is_end(&current) {
                return Some(current_distance as usize);
            }

            if prev_distance_opt.is_some() && *prev_distance_opt.unwrap() <= current_distance {
                continue;
            }

            visited.insert( current, current_distance);

            let next_dist = current_distance + 1;
            self.get_moves(&current).iter()
                .filter(|&p| {
                    if let Some(prev_dist) = visited.get(p) {
                        // Previous visit to this node was a longer path?
                        *prev_dist > next_dist
                    } else {
                        // Not visited, add to queue
                        true
                    }
                })
                .copied()
                .for_each(|next_move|
                    queue.push(Reverse((next_dist, next_move))));

        }

        None
    }
}


fn part1(input:String) -> String {
    Map::new(input).find_end().unwrap().to_string()
}

fn part2(input:String) -> String {
    Map::new(input).invert().find_end().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!("31", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_12.txt");

        assert_eq!("528", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!("29", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_12.txt");

        assert_eq!("522", solve(input.to_string(), Part2));
    }
}
