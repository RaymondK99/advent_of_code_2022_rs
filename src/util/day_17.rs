use std::collections::VecDeque;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

struct Map {
    map:VecDeque<Vec<char>>,
    jet_pattern:Vec<char>,
    index:usize,
    shapes:VecDeque<Shape>,
}

impl Map {
    fn new(jet_pattern:String) -> Map {
        let map_as_str = "+-------+";
        let mut shapes = VecDeque::new();

        shapes.push_back(Shape::build_line());
        shapes.push_back(Shape::build_plus());
        shapes.push_back(Shape::build_l());
        shapes.push_back(Shape::build_vertical());
        shapes.push_back(Shape::build_square());

        Map{map:map_as_str.lines().map(|line| line.chars().collect()).collect(),
            jet_pattern:jet_pattern.chars().collect(),
            index:0,
            shapes
        }
    }

    fn get_pos(&self, pos:&(i32,i32)) -> char {
        let (x,y) = pos;
        *self.map.get(*y as usize).unwrap().get(*x as usize).unwrap()
    }

    fn get_pos_mut(&mut self, pos:&(i32,i32)) -> &mut char {
        let (x,y) = pos;
        self.map.get_mut(*y as usize).unwrap().get_mut(*x as usize).unwrap()
    }

    fn get_start_pos(&self) -> (i32,i32) {
        (0,0)
    }

    fn prepare(&mut self) {
        let mut top_y = self.map.iter().enumerate()
            .find(|(_,line)| line.iter()
                .any(|ch| *ch == '#' || *ch == '-'))
            .map_or(self.map.len() - 1, |(y,_)| y);


        // We need at least 7 free lines in order to place a shape
        while top_y < 7 {
            self.map.push_front("|.......|".chars().collect());
            top_y += 1;
        }
    }

    fn add_shape(&mut self, shape:&Shape) {
        let points = shape.get_points();
        points.iter().for_each(|pos| *self.get_pos_mut(pos) = '#')
    }

    fn apply_pattern(&mut self, shape:&mut Shape) {
        let jet = self.jet_pattern[self.index];
        self.index = (self.index + 1) % self.jet_pattern.len();

        if jet == '<' && shape.can_move_left(self) {
            // Left
            shape.move_left();

        } else if jet == '>' && shape.can_move_right(self) {
            // Move right
            shape.move_right();
        }
    }

    fn run(&mut self, rounds:usize) -> usize {
        let start_pos = self.get_start_pos();

        for _ in 0..rounds {
            let mut shape = self.shapes.pop_front().unwrap();
            shape.set_start_pos(&start_pos);

            // Add more lines on top if needed
            self.prepare();

            loop {
                // Apply pattern
                self.apply_pattern(&mut shape);

                // Let shape drop
                if shape.can_move_down(self) {
                    shape.move_down();
                } else {
                    self.add_shape(&shape);
                    break;
                }
            }

            // Shift shapes
            self.shapes.push_back(shape);
        }

        self.get_height()
    }


    fn get_height(&self) -> usize {
        let top_y = self.map.iter().enumerate()
            .find(|(_,line)| line.iter()
                .any(|ch| *ch == '#'))
            .map_or(self.map.len() - 1, |(y,_)| y);

        self.map.len() - top_y - 1
    }

}

struct Shape {
    pos:(i32,i32), // Top left corner
    points:Vec<(i32,i32)>, // Relative positions
}

impl Shape {

    fn build_line() -> Shape {
        Shape{pos:(0,0), points:vec![(3,3),(4,3),(5,3),(6,3)]}
    }

    fn build_plus() -> Shape {
        Shape{pos:(0,0), points:vec![(3,2),(4,2),(5,2),(4,1), (4,3)]}
    }

    fn build_l() -> Shape {
        Shape{pos:(0,0), points:vec![(3,3),(4,3),(5,3),(5,2), (5,1)]}
    }

    fn build_vertical() -> Shape {
        Shape{pos:(0,0), points:vec![(3,0),(3,1),(3,2),(3,3)]}
    }

    fn build_square() -> Shape {
        Shape{pos:(0,0), points:vec![(3,2),(4,2),(3,3),(4,3)]}
    }

    fn set_start_pos(&mut self, start_pos:&(i32,i32)) {
        self.pos = *start_pos;
    }

    fn get_points(&self) -> Vec<(i32,i32)> {
        self.points.iter().map(|(x,y)| (self.pos.0 + *x, self.pos.1 + *y)).collect()
    }

    fn can_move(&self, map:&Map, delta:(i32,i32)) -> bool {
        let (dx, dy) = delta;
        self.points.iter()
            .map(|(x,y)| (*x + self.pos.0 + dx, *y + self.pos.1 + dy))
            .all(|pos| map.get_pos(&pos) == '.')
    }

    fn move_down(&mut self) {
        self.pos.1 += 1;
    }

    fn move_right(&mut self) {
        self.pos.0 += 1;
    }

    fn move_left(&mut self) {
        self.pos.0 -= 1;
    }

    fn can_move_down(&self, map:&Map) -> bool {
        self.can_move(map, (0,1))
    }

    fn can_move_right(&self, map:&Map) -> bool {
        self.can_move(map, (1,0))
    }

    fn can_move_left(&self, map:&Map) -> bool {
        self.can_move(map, (-1,0))
    }
}


fn detect_cycle(input:&Vec<usize>) -> usize {
    for cycle_size in  3..input.len() / 2 {
        for i in 0..cycle_size {
            let index = input.len() - 1 - i;
            if input[index] == input[index - cycle_size] {
                if i == cycle_size - 1 {
                    return cycle_size;
                }
            } else {
                break;
            }
        }
    }

    return 0;
}

fn part1(input : String) -> String {
    let mut map = Map::new(input);
    let height = map.run(2022);
    height.to_string()
}



fn part2(input : String) -> String {
    let input_len = input.len();
    let mut map = Map::new(input);
    let mut deltas = vec![];
    let mut num_rocks = 0;
    let mut last_height = 0;
    let mut cycle_size = 0;
    let mut min_iterations = 200;

    // Simulate enough to detect cycles
    while cycle_size == 0 || min_iterations > 0 {
        let height = map.run(input_len);
        deltas.push(height - last_height);
        last_height = height;
        num_rocks += input_len;
        min_iterations -= 1;
        cycle_size = detect_cycle(&deltas);
    }

    // Detect cycle
    let cycle_size = detect_cycle(&deltas);
    let height_per_cycle = (deltas.len() - cycle_size - 1..deltas.len() - 1)
        .map(|index| *deltas.get(index).unwrap()).sum::<usize>();

    let rocks_to_drop = 1000000000000 - num_rocks;
    let rocks_per_cycle = cycle_size * input_len;
    let num_remaining_cycles = rocks_to_drop / rocks_per_cycle;
    let rest_rocks = rocks_to_drop % rocks_per_cycle;


    (height_per_cycle * num_remaining_cycles + map.run(rest_rocks)).to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const TEST_INPUT:&str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test1() {
        assert_eq!("3068", solve(TEST_INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_17.txt");

        assert_eq!("3083", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("1514285714288", solve(TEST_INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let _input = include_str!("../../input/input_17.txt");
        //assert_eq!("1532038450107", solve(input.to_string(), Part2));
    }
}
