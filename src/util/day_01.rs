use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let calories_each:Vec<u32> = input.split("\n\n")
        .map(|item|
        item.lines()
            .map(|line| line.parse::<u32>().unwrap())
            .sum())
        .collect();

    match part {
        Part::Part1 => part1(calories_each),
        Part::Part2 => part2(calories_each)
    }
}

fn part1(calories_each:Vec<u32>) -> String {
    calories_each.iter().max().unwrap().to_string()
}


fn part2(mut calories_each:Vec<u32>) -> String {
    calories_each.sort();
    calories_each.reverse();

    [0,1,2].iter().map( |i| calories_each.get(*i as usize).unwrap()).sum::<u32>().to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!("24000", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_01.txt");

        assert_eq!("69501", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!("45000", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_01.txt");

        assert_eq!("202346", solve(input.to_string(), Part2));
    }
}
