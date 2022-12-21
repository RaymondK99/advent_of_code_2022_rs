use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

#[derive(PartialEq, Eq, Debug)]
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

fn is_in_row(n:i32, mut row:Vec<i32>) -> bool {
    let mut is_within = false;

    row.sort();
    if row.is_empty() {
        return false;
    }

    if n <= *row.first().unwrap() || n >= *row.last().unwrap() {
        return false;
    } else {
        for i in 0..row.len() - 2 {
            if row[i] != row[i+1] + 1 {
                is_within = !is_within;
                if is_within && n > row[i] && n < row[i+1] {
                    return true;
                }
            }
        }
    }

    false
}
fn is_cube_in_droplet(cube:&Cube, droplet:&Vec<Cube>) -> bool {

    // Get row
    let row = droplet.iter().filter(|p| cube.y == p.y && cube.z == p.z ).map(|p| p.x).collect::<Vec<_>>();
    let column = droplet.iter().filter(|p| cube.x == p.x && cube.z == p.z).map(|p| p.y).collect::<Vec<_>>();
    let z_row = droplet.iter().filter(|p| cube.y == p.y && cube.x == p.x).map(|p| p.z).collect::<Vec<_>>();

    let within_row = is_in_row(cube.x, row);
    let within_col = is_in_row(cube.y, column);
    let within_z_row = is_in_row(cube.z, z_row);

    within_row && within_col && within_z_row
}


fn part1(input : String) -> String {
    let points = input.lines().map(|line| Cube::new(line)).collect::<Vec<_>>();
    calculate_surface_area(&points).to_string()
}


fn part2(input : String) -> String {
    let droplet = input.lines().map(|line| Cube::new(line)).collect::<Vec<_>>();
    let surface_area = calculate_surface_area(&droplet);

    let mut inner_space = vec![];
    let min_x = droplet.iter().map(|p| p.x).min().unwrap();
    let max_x = droplet.iter().map(|p| p.x).max().unwrap();

    let min_y = droplet.iter().map(|p| p.y).min().unwrap();
    let max_y = droplet.iter().map(|p| p.y).max().unwrap();

    let min_z = droplet.iter().map(|p| p.z).min().unwrap();
    let max_z = droplet.iter().map(|p| p.z).max().unwrap();



    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                let cube = Cube::from(x, y, z);

                if !droplet.contains(&cube) {
                    // Check if cube is within droplet
                    if is_cube_in_droplet(&cube, &droplet) {
                        //println!("Cube:{:?} is in droplet", cube);
                        inner_space.push(cube);
                    }
                } else {
                    //println!("Cube is in droplet:{:?}", cube);
                }
            }
        }
    }

    let mut inner_area = 0;
    for inner_cube in inner_space.iter() {
        for cube_in_droplet in droplet.iter() {
            if inner_cube.is_connected(cube_in_droplet) {
                inner_area += 1;
            }
        }
    }

    let surface_area_inner_cube = calculate_surface_area(&inner_space);
    println!("inner cube area={}", surface_area_inner_cube);

    println!("inner area={}", inner_area);
    println!("total area={}", surface_area);
    println!("outer surface={}", surface_area - inner_area);

    (surface_area - inner_area).to_string()
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
        // to low
        // 2530
        // to high:
        // 4220
        assert_eq!("64", solve(TEST_INPUT.to_string(), Part2));
    }

    //#[test]
    fn _test_part2() {
        let input = include_str!("../../input/input_18.txt");

        assert_eq!("1", solve(input.to_string(), Part2));
    }
}
