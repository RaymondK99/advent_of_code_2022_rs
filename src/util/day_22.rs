use std::collections::{HashMap, VecDeque};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    Left = 2,
    Right = 0,
    Up = 3,
    Down = 1,
    None = 5,
}

impl Direction {
    fn turn(&self,dir:Direction) -> Direction  {
        if dir == Direction::Left {
            match *self {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                _ => Direction::None,
            }
        } else if dir == Direction::Right {
            match *self {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                _ => Direction::None,
            }
        } else {
            *self
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Pos {
    x:i32,
    y:i32,
}

impl Pos {
    fn new(x:i32, y:i32) -> Pos {
        Pos{x,y}
    }

    fn next_pos(&self, dir:Direction) -> Pos {
        match dir {
            Direction::Left=> Pos::new(self.x-1,self.y),
            Direction::Right=> Pos::new(self.x+1,self.y),
            Direction::Up => Pos::new(self.x,self.y-1),
            Direction::Down => Pos::new(self.x,self.y+1),
            Direction::None => Pos::new(self.x, self.y),
        }
    }
}

#[derive(Debug)]
struct Map {
    map:HashMap<Pos, char>,
    max_y:i32,
    max_x:i32,
}

impl Map {
    fn parse(input:&str) -> Map {
        let mut map = HashMap::new();
        input.lines().enumerate()
            .map(|(y, line)| line.chars().enumerate()
                .map(move |(x, ch)| (x,y, ch))).flatten()
            .for_each( |(x,y,ch)| {
                map.insert(Pos::new(x as i32,y as i32), ch);
            });

        let max_x = map.iter().map(|(p,_)| p.x).max().unwrap();
        let max_y = map.iter().map(|(p,_)| p.y).max().unwrap();
        Map{map, max_x, max_y}
    }

    fn get_start_pos(&self) -> Pos {
        let y = self.map.iter().filter(|(_,&ch)| ch == '.').map(|(p,_)| p.y).min().unwrap();
        let x = self.map.iter().filter(|(pos,&ch)| ch == '.' && pos.y == y).map(|(p,_)| p.x).min().unwrap();
        Pos::new(x,y)
    }

    fn get_next(&self, pos:Pos, dir:Direction) -> (Pos, char) {
        let mut next = pos.next_pos(dir);

        loop {
            if let Some(ch) = self.map.get(&next) {
                if *ch == '.' || *ch == '#' || *ch == '*' {
                    return (next, *ch)
                } else {
                    next = next.next_pos(dir);
                    continue;
                }
            } else {
                // Ended up outside the map
                if dir == Direction::Right {
                    next.x = 0;
                } else if dir == Direction::Down {
                    next.y = 0;
                } else if dir == Direction::Up {
                    if next.y <= 0 {
                        next.y = self.max_y;
                    } else {
                        next.y -= 1;
                    }
                } else {
                    // Left
                    if next.x <= 0 {
                        next.x = self.max_x;
                    } else {
                        next.x -= 1;
                    }
                }
            }
        }
    }

    fn _print(&self) {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                print!("{}", self.map.get(&Pos::new(x,y)).unwrap_or(&' '));
            }
            println!();
        }
    }
    fn execute_path(&mut self, path:&mut VecDeque<char>) -> (Pos, Direction) {
        let mut current_direction = Direction::Right;
        let mut current_pos = self.get_start_pos();

        *self.map.get_mut(&current_pos).unwrap() = '*';
        while !path.is_empty() {
            let (next_turn, mut steps) = get_next_move(path);
            //println!("next turn:{:?}, steps:{}, current_dir:{:?}", next_turn, steps, current_direction);
            while steps > 0 {
                let (next_pos, ch) = self.get_next(current_pos, current_direction);

                //println!("next pos:{:?}, dir={:?}, ch={}", next_pos, current_direction, ch);
                if ch == '.' || ch == '*' {
                    // Ok move
                    current_pos = next_pos;
                    *self.map.get_mut(&current_pos).unwrap() = '*';
                } else {
                    // Stop
                    break;
                }

                steps -= 1;
            }

            // Face next direction
            current_direction = current_direction.turn(next_turn);
        }

        (current_pos, current_direction)
    }
}

fn get_next_move(path:&mut VecDeque<char>) -> (Direction, i32) {
    if path.is_empty() {
        panic!("....");
    } else {
        let mut number = String::new();
        while !path.is_empty() && path.front().unwrap().is_digit(10) {
            number.push(path.pop_front().unwrap());
        }

        let dir = if let Some(ch) = path.pop_front() {
            match ch {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => { panic!("...");}
            }
        } else {
            Direction::None
        };

        (dir, number.parse().unwrap())
    }
}

fn part1(input : String) -> String {
    let mut it = input.split("\n\n");
    let mut map = Map::parse(it.next().unwrap());
    let mut path = it.next().unwrap().chars().collect::<VecDeque<_>>();

    //map.print();
    let (pos, direction) = map.execute_path(&mut path);
    //map.print();

    println!("pos:{:?}, dir:{:?}", pos, direction as i32);

    ((pos.x+1) * 4 + (pos.y+1) * 1000 + direction as i32).to_string()
}

fn part2(_input : String) -> String {
    "1".to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const TEST_INPUT:&str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test1() {
        assert_eq!("6032", solve(TEST_INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_22.txt");
        assert_eq!("3590", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "";

        assert_eq!("1", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_22.txt");

        assert_eq!("1", solve(input.to_string(), Part2));
    }
}
