use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

fn part1(_input : String) -> String {
    "1".to_string()
}


fn part2(_input : String) -> String {
    "1".to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "";

        assert_eq!("1", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_02.txt");

        assert_eq!("1", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "";

        assert_eq!("1", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_02.txt");

        assert_eq!("1", solve(input.to_string(), Part2));
    }
}
