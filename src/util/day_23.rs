use std::collections::{HashMap, HashSet, VecDeque};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}


#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Position {
    x:i32,
    y:i32,
}

struct Grid {
    elves:HashSet<Position>,
}

impl Position {
    fn new(x:i32,y:i32) -> Position {
        Position {x,y}
    }

    fn adjacent_positions(&self) -> [Position;8] {
        [Position::new(self.x-1, self.y-1),
            Position::new(self.x, self.y-1),
            Position::new(self.x+1, self.y-1),
            Position::new(self.x-1, self.y+1),
            Position::new(self.x, self.y+1),
            Position::new(self.x+1, self.y+1),
            Position::new(self.x-1, self.y),
            Position::new(self.x+1, self.y)]
    }

    fn next_move(&self, dir:Direction) -> Position {
        match dir {
            Direction::North => Position::new(self.x, self.y-1),
            Direction::South => Position::new(self.x, self.y+1),
            Direction::West => Position::new(self.x-1, self.y),
            Direction::East => Position::new(self.x+1, self.y),
        }
    }

    fn check_direction(&self, dir:Direction) -> [Position; 3] {
        match dir {
            Direction::North => [Position::new(self.x-1, self.y-1), Position::new(self.x, self.y-1), Position::new(self.x+1, self.y-1)],
            Direction::South => [Position::new(self.x-1, self.y+1), Position::new(self.x, self.y+1), Position::new(self.x+1, self.y+1)],
            Direction::West =>  [Position::new(self.x-1, self.y-1), Position::new(self.x-1, self.y), Position::new(self.x-1, self.y+1)],
            Direction::East =>  [Position::new(self.x+1, self.y-1), Position::new(self.x+1, self.y), Position::new(self.x+1, self.y+1)],
        }
    }
}

impl Grid {
    fn new(input:String) -> Grid {
        let elves = input.lines().enumerate()
            .map(|(y, line)| line.chars().enumerate()
                .map(move |(x,ch)| (x,y,ch)))
            .flatten()
            .filter(|(_,_,ch)| *ch == '#')
            .map(|(x,y,_)| Position::new(x as i32, y as i32))
            .collect();
        Grid{elves}
    }

    fn perform_move(&mut self, rounds:usize) -> usize {
        let mut round = 0;

        while round < rounds {
            let mut proposed_moves = VecDeque::new();
            let mut conflicting_moves = HashMap::new();
            for elf in self.elves.iter() {
                // Should move
                let any_adjacent = elf.adjacent_positions().iter().any(|adjacent_pos| self.elves.contains(adjacent_pos));
                if !any_adjacent {
                    // No adjacent elves, skip move
                    continue;
                }

                // proposed move
                for move_no in 0..4 {
                    let dir_no = (round + move_no) % 4;
                    let dir = match dir_no {
                        0 => Direction::North,
                        1 => Direction::South,
                        2 => Direction::West,
                        3 => Direction::East,
                        _ => panic!("..."),
                    };

                    let any_blocking_elf = elf.check_direction(dir).iter().any(|pos| self.elves.contains(pos));
                    if !any_blocking_elf {
                        let next_move = elf.next_move(dir);
                        proposed_moves.push_back((elf.clone(),next_move.clone()));
                        if let Some(cnt) = conflicting_moves.get_mut(&elf.next_move(dir)) {
                            *cnt += 1;
                        } else {
                            conflicting_moves.insert(next_move, 1);
                        }

                        break;
                    }
                }
            }

            if proposed_moves.is_empty() {
                // No moves
                break;
            } else {
                while !proposed_moves.is_empty() {
                    // Perform moves
                    let (elf, next_move) = proposed_moves.pop_front().unwrap();

                    if *conflicting_moves.get(&next_move).unwrap() > 1 {
                        // Skip
                        continue;
                    } else {
                        // Perform move
                        self.elves.remove(&elf);
                        self.elves.insert(next_move);
                    }
                }
            }

            round +=1;
        }

        round+1
    }

    fn num_elves(&self) -> usize {
        self.elves.len()
    }

    fn get_area(&self) -> usize {
        let y_max = self.elves.iter().map(|e| e.y).max().unwrap();
        let x_max = self.elves.iter().map(|e| e.x).max().unwrap();
        let y_min = self.elves.iter().map(|e| e.y).min().unwrap();
        let x_min = self.elves.iter().map(|e| e.x).min().unwrap();
        ((x_max - x_min + 1) * (y_max - y_min + 1)) as usize
    }
}

fn part1(input : String) -> String {
    let mut grid = Grid::new(input);
    grid.perform_move(10);
    (grid.get_area() - grid.num_elves()).to_string()
}

fn part2(input : String) -> String {
    let mut grid = Grid::new(input);
    grid.perform_move(1_000_000).to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const TEST_INPUT1:&str = ".....
..##.
..#..
.....
..##.
.....";

    const TEST_INPUT2:&str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";


    #[test]
    fn test11() {
        assert_eq!("25", solve(TEST_INPUT1.to_string(), Part1));
    }

    #[test]
    fn test12() {
        assert_eq!("110", solve(TEST_INPUT2.to_string(), Part1));
    }


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_23.txt");
        assert_eq!("3757", solve(input.to_string(), Part1));
    }

    #[test]
    fn test21() {
        assert_eq!("4", solve(TEST_INPUT1.to_string(), Part2));
    }

    #[test]
    fn test22() {
        assert_eq!("20", solve(TEST_INPUT2.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_23.txt");

        assert_eq!("918", solve(input.to_string(), Part2));
    }
}
