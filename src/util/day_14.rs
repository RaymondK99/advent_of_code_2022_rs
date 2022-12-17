use std::collections::VecDeque;
use std::mem::swap;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}


fn parse(line:&str) -> Vec<(usize,usize)> {
    line.split(" -> ")
        .map(|item| {
            let mut it = item.split(',').into_iter();
            let x = it.next().unwrap().parse::<usize>().unwrap();
            let y = it.next().unwrap().parse::<usize>().unwrap();
            (x,y)
        })
        .collect::<Vec<_>>()
}

fn build_map(positions:Vec<Vec<(usize,usize)>>) -> ((usize, usize),Vec<Vec<char>>) {
    let max_y = positions.iter().flat_map(|line| line.iter()).map(|(_,y)| *y).max().unwrap();
    let max_x = positions.iter().flat_map(|line| line.iter()).map(|(x,_)| *x).max().unwrap();
    let min_x = positions.iter().flat_map(|line| line.iter()).map(|(x,_)| *x).min().unwrap();

    let width = max_x - min_x + 3;
    let height = max_y + 2;

    let mut normalized_positions = positions.iter()
        .map(|line| line.iter()
            .map(|(x,y)| (*x - min_x+1, *y)).collect::<VecDeque<_>>())
        .collect::<VecDeque<_>>();

    let mut map = vec![];
    for _ in 0..height {
        let mut line = vec![];
        for _ in 0..width {
            line.push('.');
        }
        map.push(line);
    }

    while !normalized_positions.is_empty() {
        let mut lines = normalized_positions.pop_front().unwrap();

        while lines.len() > 1 {
            let (mut x0, mut y0) = lines.pop_front().unwrap();
            let (mut x1, mut y1) = *lines.front().unwrap();

            if x0 > x1 {
                swap(&mut x0, &mut x1);
            }

            if y0 > y1 {
                swap(&mut y0, &mut y1);
            }

            for y in y0..=y1 {
                for x in x0..=x1 {
                    //println!("{},{}",x,y);
                    *map.get_mut(y).unwrap().get_mut(x).unwrap() = '#';
                }
            }
        }
    }

    ((500 - min_x + 1, 0), map)
}

fn drop_sand(pos:(usize,usize), map:&mut Vec<Vec<char>>, part_two:bool) -> bool {
    let mut current_pos = pos;
    let height = map.len();
    let width = map.first().unwrap().len();

    loop {
        let (x,y) = current_pos;
        let mut next = vec![];

        // Try down
        if y == height - 1 {
            if !part_two {
                return true;
            }
        } else {
            next.push((x,y+1));
        }

        // try left-down
        if x > 0 && y < height - 1 {
            next.push((x-1,y+1))
        }

        // try down-right
        if x < width - 1 && y < height - 1 {
            next.push((x+1,y+1))
        }

        if let Some(next_pos)  = next.iter().find(|&next_pos| get_pos(*next_pos, map) == '.') {
            // Sand moved to next pos
            current_pos = *next_pos;
            continue;
        } else {
            // Sand stays
            let current_item = map.get_mut(y).unwrap().get_mut(x).unwrap();

            // Is the current spot occupied already?
            if *current_item == '.' {
                *current_item = 'o';
                return false;
            } else {
                return true;
            }
        }
    }
}

fn print(map:&Vec<Vec<char>>) {
    for y in 0..map.len() {
        let line = map.get(y).unwrap();
        for x in 0..line.len() {
            print!("{}", line.get(x).unwrap());
        }
        println!();
    }
}

fn get_pos(pos:(usize,usize), map:&Vec<Vec<char>> ) -> char {
    let (x,y) = pos;
    *map.get(y).unwrap().get(x).unwrap()
}

fn part1(input : String) -> String {
    let positions = input.lines().map(|line| parse(line)).collect::<Vec<_>>();
    let (start_pos, mut map) = build_map(positions);

    print(&map);
    while !drop_sand(start_pos, &mut map, false) {}

    print(&map);
    map.iter().flat_map(|line| line.iter()).filter(|&ch| *ch == 'o').count().to_string()
}

fn part2(input : String) -> String {
    let positions = input.lines().map(|line| parse(line)).collect::<Vec<_>>();
    let (start_pos, mut map) = build_map(positions);

    while !drop_sand(start_pos, &mut map, true) {}

    // Trim line with leading and trailing '.'
    map.iter_mut().for_each(|line| {
        while *line.last().unwrap() == '.' {
            line.pop();
        }

        line.reverse();

        while *line.last().unwrap() == '.' {
            line.pop();
        }
    });

    // Count non sand items within the area
    let space_or_wall_items = map.iter().flat_map(|line| line.iter()).filter(|ch| **ch != 'o').count();

    // Total area of triangle minus non sand items
    (map.len() * map.len() - space_or_wall_items).to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!("24", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_14.txt");

        assert_eq!("1513", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!("93", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_14.txt");

        assert_eq!("22646", solve(input.to_string(), Part2));
    }
}
