use std::collections::{HashSet, VecDeque};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct Cube {
    x:i32,
    y:i32,
    z:i32,
}

impl Cube {
    fn new(line:&str) -> Cube {
        let mut it = line.split(',').map(|col| col.parse::<i32>().unwrap());
        Cube {x:it.next().unwrap(), y:it.next().unwrap(), z:it.next().unwrap()}
    }

    fn from(x:i32,y:i32,z:i32) -> Cube {
        Cube {x,y,z}
    }

    fn is_connected(&self, other:&Cube) -> bool {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)  == 1
    }

    fn get_neighbors(&self) -> Vec<Cube> {
        let mut neighbors = vec![];
        neighbors.push(Cube::from(self.x-1, self.y, self.z));
        neighbors.push(Cube::from(self.x+1, self.y, self.z));
        neighbors.push(Cube::from(self.x, self.y-1, self.z));
        neighbors.push(Cube::from(self.x, self.y+1, self.z));
        neighbors.push(Cube::from(self.x, self.y, self.z+1));
        neighbors.push(Cube::from(self.x, self.y, self.z-1));
        neighbors
    }
}

fn calculate_surface_area(cubes:&Vec<Cube>) -> i32 {
    let mut exposed_area = 0;
    for i in 0..cubes.len() {
        let mut connections = 0;
        for j in 0..cubes.len() {
            if i != j &&  cubes[i].is_connected(&cubes[j]) {
                connections += 1;
            }
        }
        exposed_area += 6 - connections;
    }

    exposed_area
}


fn calculate_outer_surface(droplet:HashSet<Cube>) -> usize {
    let mut outer_formation = HashSet::new();

    let min_x = droplet.iter().map(|p| p.x).min().unwrap() - 1 ;
    let max_x = droplet.iter().map(|p| p.x).max().unwrap() + 1;

    let min_y = droplet.iter().map(|p| p.y).min().unwrap() - 1;
    let max_y = droplet.iter().map(|p| p.y).max().unwrap() + 1;

    let min_z = droplet.iter().map(|p| p.z).min().unwrap() - 1;
    let max_z = droplet.iter().map(|p| p.z).max().unwrap() + 2;
    let first = Cube::from(min_x, min_y, min_z);

    let mut queue = VecDeque::new();
    queue.push_back(first);

    while !queue.is_empty() {
        let cube = queue.pop_front().unwrap();

        if cube.x < min_x || cube.x > max_x || cube.y < min_y || cube.y > max_y || cube.z < min_z || cube.z > max_z {
            continue;
        }

        if droplet.contains(&cube) || outer_formation.contains(&cube) {
            continue;
        }

        // Add neighbours for eval
        cube.get_neighbors().into_iter().for_each(|c| queue.push_back(c));

        // Add to outer
        outer_formation.insert(cube);
    }

    let mut connections = 0;
    for c in outer_formation {
        for d in droplet.iter() {
            if c.is_connected(&d) {
                connections += 1;
            }
        }
    }

    connections
}


fn part1(input : String) -> String {
    let points = input.lines().map(|line| Cube::new(line)).collect();
    calculate_surface_area(&points).to_string()
}

fn part2(input : String) -> String {
    let droplet = input.lines().map(|line| Cube::new(line)).collect();
    calculate_outer_surface(droplet).to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const TEST_INPUT:&str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test1() {
        assert_eq!("64", solve(TEST_INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_18.txt");
        assert_eq!("4450", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("58", solve(TEST_INPUT.to_string(), Part2));
    }

    #[test]
    fn _test_part2() {
        let input = include_str!("../../input/input_18.txt");
        assert_eq!("2564", solve(input.to_string(), Part2));
    }
}
