use std::collections::{HashSet, VecDeque};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

struct Map {
    map:Vec<Vec<u8>>,
}

impl Map {
    const FREE:u8 = 0b0;
    const UP:u8 = 0b1;
    const DOWN:u8 = 0b10;
    const LEFT:u8 = 0b100;
    const RIGHT:u8 = 0b1000;
    const WALL:u8 = 0b10000;

    fn parse(input:String) -> Map {
        Map{map:input.lines().map(|row|
            row.chars().map(|ch| {
                match ch {
                    '.' => Map::FREE,
                    '#' => Map::WALL,
                    '^' => Map::UP,
                    'v' => Map::DOWN,
                    '<' => Map::LEFT,
                    '>' => Map::RIGHT,
                    _ => panic!("unexpected char:{}",{ch}),
                }
            }).collect())
            .collect()
        }
    }

    fn move_blizzards(&mut self) {
        let mut next_positions = vec![];
        let max_y = self.map.len() - 2;
        let max_x = self.map.first().unwrap().len() - 2;
        for y in 1..=max_y {
            let row = self.map.get_mut(y).unwrap();
            for x in 1..=max_x {
                let item = row.get_mut(x).unwrap();
                if *item != Map::FREE {
                    let blizzards = [Map::UP, Map::DOWN, Map::LEFT, Map::RIGHT];
                    for blizz in blizzards {
                        if let Some((next_x, next_y)) = match blizz & *item {
                            Map::UP => {
                                if y == 1 {
                                    Some((x, max_y))
                                } else {
                                    Some((x, y - 1))
                                }
                            },
                            Map::DOWN => {
                                if y == max_y {
                                    Some((x, 1))
                                } else {
                                    Some((x, y + 1))
                                }
                            },
                            Map::LEFT => {
                                if x == 1 {
                                    Some((max_x, y))
                                } else {
                                    Some((x - 1, y))
                                }
                            },
                            Map::RIGHT => {
                                if x == max_x {
                                    Some((1, y))
                                } else {
                                    Some((x + 1, y))
                                }
                            },
                            _ => None,
                        } {
                            next_positions.push((blizz & *item, next_x, next_y));
                        }
                    }
                    // Clear position
                    *item = Map::FREE;
                }
            }
        }

        // Add next positions
        for (blizz, next_x, next_y) in next_positions {
            *self.map.get_mut(next_y).unwrap().get_mut(next_x).unwrap() |= blizz;
        }
    }

    fn get_exit(&self) -> (i32,i32) {
        (self.map.first().unwrap().len() as i32 - 2,self.map.len() as i32 - 1)
    }

    fn get_start(&self) -> (i32,i32) {
        (1,0)
    }

    fn find_shortest_path(&mut self, start_pos:(i32,i32), end_pos:(i32, i32)) -> usize {
        let mut queue = VecDeque::new();
        let mut blizzard_step = 0;
        let mut visited = HashSet::new();
        let max_y = self.map.len() as i32 - 1;

        queue.push_back((0,start_pos.0,start_pos.1));

        while !queue.is_empty() {
            let (step_no, x,y) = queue.pop_front().unwrap();
            if visited.contains(&(step_no, x , y)) {
                continue;
            } else {
                visited.insert((step_no, x, y));
            }

            if (x,y).eq(&end_pos) {
                // Found solution
                return blizzard_step - 1
            }

            // Check next moves, move blizzards
            if step_no != blizzard_step {
                self.move_blizzards();
                blizzard_step = step_no;
                visited.clear();
            }

            let candidate_positions = [(x,y), (x+1,y),(x-1,y),(x,y-1),(x,y+1)];
            candidate_positions.iter()
                .filter(|(x,y)| *x >= 0 && *y >= 0 && *y <= max_y && self.get_pos(*x,*y) == Map::FREE)
                .for_each(|(x,y)| {
                    queue.push_back((blizzard_step+1, *x, *y));
                })
        }

        panic!("No solution");
    }

    fn get_pos(&self,x:i32, y:i32) -> u8 {
        *self.map.get(y as usize).unwrap().get(x as usize).unwrap()
    }
}

fn part1(input : String) -> String {
    let mut map = Map::parse(input);
    map.find_shortest_path(map.get_start(), map.get_exit()).to_string()
}

fn part2(input : String) -> String {
    let mut map = Map::parse(input);
    let first_path = map.find_shortest_path(map.get_start(), map.get_exit());
    let second_path = map.find_shortest_path(map.get_exit(), map.get_start()) + 1;
    let third_path = map.find_shortest_path(map.get_start(), map.get_exit()) + 1;
    (first_path + second_path + third_path).to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const TEST_INPUT1:&str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test1() {
        assert_eq!("18", solve(TEST_INPUT1.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_24.txt");
        assert_eq!("290", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("54", solve(TEST_INPUT1.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_24.txt");
        assert_eq!("842", solve(input.to_string(), Part2));
    }
}
