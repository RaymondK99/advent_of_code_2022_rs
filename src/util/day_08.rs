use super::Part;

pub fn solve(input : String, part: Part) -> String {
    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    fn get_next(&self) -> (i32,i32) {
        match *self {
            Direction::UP => (0,-1),
            Direction::DOWN => (0,1),
            Direction::LEFT => (-1,0),
            Direction::RIGHT => (1,0),
        }
    }
}

struct Forest {
    trees:Vec<Vec<u32>>,
    size:i32,
}

impl Forest {

    fn new(input:String) -> Forest {
        let trees:Vec<Vec<u32>> = input.lines()
            .map( |line| line.as_bytes().iter().map(|b| (*b - b'0') as u32).collect::<Vec<u32>>())
            .collect();
        let size = trees.len() as i32;
        Forest{trees, size}
    }

    fn get_tree_height(&self, x:i32,y:i32) -> u32 {
        *self.trees.get(y as usize).unwrap().get(x as usize).unwrap()
    }

    fn get_next(&self, pos:(i32, i32), dir:Direction) -> Option<(i32, i32, u32)> {
        let (xd,yd) = dir.get_next();
        let x = pos.0 + xd;
        let y = pos.1 + yd;
        if y < 0 || x < 0 || y >= self.size || x >= self.size {
            None
        } else {
            let height = self.get_tree_height(x,y);
            Some((x,y,height))
        }
    }


    fn is_visible(&self, pos: &(i32, i32)) -> bool {
        [Direction::DOWN, Direction::UP, Direction::LEFT, Direction::RIGHT]
            .iter().any(|dir| self.is_visible_direction(pos, *dir))
    }

    fn is_visible_direction(&self, pos:&(i32, i32), direction:Direction) -> bool {
        let (mut x, mut y) = pos;
        let height = self.get_tree_height(x, y);

        loop {
            if let Some((x_next,y_next, next_height)) = self.get_next((x, y), direction) {
                if next_height < height {
                    // continue
                    x = x_next;
                    y = y_next;
                } else {
                    return false
                }
            } else {
                return true
            }
        }
    }

    fn get_visibility_score(&self, pos: &(i32, i32)) -> i32 {
        [Direction::DOWN, Direction::UP, Direction::LEFT, Direction::RIGHT]
            .iter().map(|dir| self.get_visibility(&pos, *dir)).product()
    }

    fn get_visibility(&self, pos: &(i32, i32), direction:Direction) -> i32 {
        let (mut x, mut y) = pos;
        let height = self.get_tree_height(x, y);
        let mut acc = 0;

        loop {
            if let Some((x_next,y_next, next_height)) = self.get_next((x, y), direction) {
                acc += 1;
                if next_height >= height {
                    break;
                } else {
                    // continue
                    x = x_next;
                    y = y_next;
                }
            } else {
                break
            }
        }
        acc
    }

    fn get_positions(&self) -> Vec<(i32,i32)> {
        (0..self.size).into_iter()
            .flat_map(|y| (0..self.size).into_iter()
                .map(move |x|(x,y)))
            .collect()
    }
}


fn part1(input : String) -> String {
    let forest = Forest::new(input);
    forest.get_positions().iter().filter(|&pos| forest.is_visible(pos)).count().to_string()
}

fn part2(input : String) -> String {
    let forest = Forest::new(input);
    forest.get_positions().iter().map(|pos| forest.get_visibility_score(pos)).max().unwrap().to_string()
}


#[cfg(test)]
mod tests {
// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;
use util::Part::{Part1, Part2};


#[test]
fn test1() {

    let input = "30373
25512
65332
33549
35390";

    assert_eq!("21", solve(input.to_string(), Part1));
}

#[test]
fn test_part1() {
    let input = include_str!("../../input/input_08.txt");

    assert_eq!("1832", solve(input.to_string(), Part1));
}

#[test]
fn test2() {

    let input = "30373
25512
65332
33549
35390";

    assert_eq!("8", solve(input.to_string(), Part2));
}

#[test]
fn test_part2() {
    let input = include_str!("../../input/input_08.txt");

    assert_eq!("157320", solve(input.to_string(), Part2));
}
}
