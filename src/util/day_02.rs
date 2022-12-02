use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Type {
    PAPER,
    ROCK,
    SCISSORS,
}

fn get_points(m:Type) -> u32 {
    match m {
        Type::PAPER => 2,
        Type::ROCK => 1,
        Type::SCISSORS=> 3,
    }
}

fn parse_opponent_move(ch: char) -> Type {
    match ch {
        'A' => Type::ROCK,
        'B' => Type::PAPER,
        'C' => Type::SCISSORS,
        _ => {
            panic!("unknown type");
        }
    }
}

fn parse_my_move_part1(_opponent:&Type, ch: char) -> Type {
    match ch {
        'X' => Type::ROCK,
        'Y' => Type::PAPER,
        'Z' => Type::SCISSORS,
        _ => {
            panic!("unknown type");
        }
    }
}

fn parse_my_move_part2(opponent:&Type, my_move:char) -> Type {
    match my_move {
        // Should loose
        'X' => match opponent {
            Type::PAPER => Type::ROCK,
            Type::ROCK => Type::SCISSORS,
            Type::SCISSORS => Type::PAPER,
            },
        // Draw
        'Y' => *opponent,
        // Should win
        'Z' => match opponent {
            Type::PAPER => Type::SCISSORS,
            Type::ROCK => Type::PAPER,
            Type::SCISSORS => Type::ROCK,
        },
        _ => {
            panic!("invalid input")
        }
    }
}

fn game_points(opponent:&Type, my_move:&Type) -> u32 {
    if *opponent == Type::ROCK {
        match my_move {
            Type::ROCK => 3,
            Type::SCISSORS => 0,
            Type::PAPER => 6,
        }
    } else if *opponent == Type::PAPER {
        match my_move {
            Type::ROCK => 0,
            Type::SCISSORS => 6,
            Type::PAPER => 3,
        }
    } else {
        match my_move {
            Type::ROCK => 6,
            Type::SCISSORS => 3,
            Type::PAPER => 0,
        }
    }
}

fn play(lines : Vec<&str>, parse_my_move: &dyn Fn(&Type, char) -> Type) -> String {
    lines.iter().map( |line| {
        let chars:Vec<char> = line.to_string().chars().collect();
        let opponent = parse_opponent_move(chars[0]);
        let my_move = parse_my_move(&opponent, chars[2]);
        game_points(&opponent, &my_move) + get_points(my_move)
    }).sum::<u32>().to_string()
}

fn part1(lines : Vec<&str>) -> String {
    play(lines, &parse_my_move_part1)
}

fn part2(lines : Vec<&str>) -> String {
    play(lines, &parse_my_move_part2)
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "A Y
B X
C Z";

        assert_eq!("15", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_02.txt");

        assert_eq!("9241", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "A Y
B X
C Z";

        assert_eq!("12", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_02.txt");

        assert_eq!("14610", solve(input.to_string(), Part2));
    }
}
