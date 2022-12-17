use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input, 2000000),
        Part::Part2 => part2(input, 4000000)
    }
}

#[derive(Debug, Copy, Clone)]
struct Sensor {
    position:Pos,
    closest_beacon:Pos,
}

impl Sensor {
    fn new(coord:&[i32]) -> Sensor {
        Sensor{position:Pos{x: coord[0], y: coord[1]}, closest_beacon:Pos{x: coord[2], y: coord[3]}}
    }

    fn get_range_for_y(&self, y:i32) -> Option<(i32,i32)> {
        let dist = self.position.dist(&self.closest_beacon);
        let y_dist = y.abs_diff(self.position.y);

        if y_dist > dist {
            None
        } else {
            let delta = y_dist.abs_diff(dist) as i32;
            Some((self.position.x - delta, self.position.x + delta))
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Pos {
    x:i32,
    y:i32,
}

impl Pos {

    fn dist(&self,other:&Pos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn parse_line(line:&str) -> Sensor {
    let columns = line.split([' ',':',','])
        .filter(|col| !col.is_empty() && col.contains("="))
        .map(|col| col.split('=').last().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    Sensor::new(columns.as_slice())
}

fn count_points_in_ranges(mut ranges:Vec<(i32, i32)>) -> usize {

    let mut points = 0;
    let mut last_interval = None;

    while !ranges.is_empty() {
        let (min_x, max_x) = ranges.pop().unwrap();
        let interval_size = max_x - min_x + 1;
        if last_interval.is_none() {
            points += interval_size;
        } else {
            let (_, last_max_x) = last_interval.unwrap();

            // Overlap ?
            if last_max_x >= min_x {
                let overlap = last_max_x - min_x + 1;
                let delta = interval_size - overlap;
                if overlap > interval_size {
                    continue;
                }
                points += delta;
            } else {
                points += interval_size;
            }
        }

        last_interval = Some((min_x, max_x));
    }

    points as usize
}

fn find_non_overlap(y:i32, min_coord:i32, max_coord:i32, mut ranges:Vec<(i32, i32)>) -> Option<(i32, i32)> {

    let mut last_interval = None;

    while !ranges.is_empty() {
        let (min_x, max_x) = ranges.pop().unwrap();

        if last_interval.is_some() {
            let (_, last_max_x) = last_interval.unwrap();

            // Is this range consumed by the last?
            if last_max_x >= max_x {
                continue;
            }

            // Is there a cap if at least 1?
            if (min_x - last_max_x) > 1 && min_x > min_coord && min_x < max_coord {
                // Found glitch
                return Some((last_max_x + 1,y));
            }
        }

        last_interval = Some((min_x, max_x));
    }

    None
}

fn part1(input : String, for_y:i32) -> String {
    let sensors = input.lines().map(|line| parse_line(line)).collect::<Vec<_>>();

    let mut ranges_for_y_coord = sensors.iter()
        .map(|sensor| sensor.get_range_for_y(for_y))
        .filter(|range| range.is_some())
        .map(|range| range.unwrap())
        .collect::<Vec<_>>();

    ranges_for_y_coord.sort();
    ranges_for_y_coord.reverse();

    let num_points = count_points_in_ranges(ranges_for_y_coord);

    let mut beacons_for_y = sensors.iter()
        .map(|sensor| sensor.closest_beacon)
        .filter(|beacon| beacon.y == for_y)
        .collect::<Vec<_>>();

    beacons_for_y.sort();
    beacons_for_y.dedup();

    (num_points - beacons_for_y.len()).to_string()
}

fn part2(input : String, max_coord:i32) -> String {
    let sensors = input.lines().map(|line| parse_line(line)).collect::<Vec<_>>();

    let (x,y) = (0..=max_coord).into_iter().map(|y| {
        let mut ranges_for_y_coord = sensors.iter()
            .map(|sensor| sensor.get_range_for_y(y))
            .filter(|range| range.is_some())
            .map(|range| range.unwrap())
            .collect::<Vec<_>>();

        ranges_for_y_coord.sort();
        ranges_for_y_coord.reverse();

        find_non_overlap(y, 0, max_coord, ranges_for_y_coord)
    }).find(|coord| coord.is_some())
        .map(|res| res.unwrap())
        .unwrap();

    (x as usize * 4000000 + y as usize).to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test1() {
        assert_eq!("26", part1(TEST_INPUT.to_string(), 10));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_15.txt");

        assert_eq!("4827924", part1(input.to_string(), 2000000));
    }

    #[test]
    fn test2() {
        assert_eq!("56000011", part2(TEST_INPUT.to_string(), 20));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_15.txt");

        assert_eq!("12977110973564", part2(input.to_string(), 4000000));
    }
}
