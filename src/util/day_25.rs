use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

fn snafu_to_decimal(input:&str) -> i64 {
    input.chars().enumerate().map(|(i, ch)| {
        let exp = input.len() - i - 1;
        let snafu_value = match ch {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unexpected char"),
        };
        snafu_value * i64::pow(5, exp as u32)
    }).sum()
}

fn max_value(digits:usize) -> i64 {
    (0..digits).into_iter().map(|n| 2 * i64::pow(5, n as u32)).sum()
}

fn get_number_of_snafu_digits(decimal_value:i64) -> usize {
    let mut num_digits = 1;
    loop {
        if max_value(num_digits) > decimal_value {
            return num_digits
        } else {
            num_digits += 1;
        }
    }
}

fn decimal_to_snafu(decimal_value:i64) -> String {
    decimal_to_snafu_rec(get_number_of_snafu_digits(decimal_value), decimal_value, vec![])
}

fn decimal_to_snafu_rec(snafu_digit_no:usize, decimal_value:i64, mut result:Vec<char>) -> String {
    if snafu_digit_no == 0 {
        return result.iter().collect();
    };

    let mut min_diff = u64::MAX;
    let mut next_snafu_value = 0;
    let mut rest = 0;
    for a in [2,1,0,-1,-2] {
        let snafu_value = a * i64::pow(5, snafu_digit_no as u32 - 1);
        if decimal_value.abs_diff(snafu_value) < min_diff {
            min_diff = decimal_value.abs_diff(snafu_value);
            next_snafu_value = a;
            rest = decimal_value - snafu_value;
        }
    }

    result.push(match next_snafu_value {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!("..."),
    });

    return decimal_to_snafu_rec(snafu_digit_no - 1, rest, result);
}


fn part1(input : String) -> String {
    let decimal_sum_value = input.lines()
        .map(|line| snafu_to_decimal(line))
        .sum();
    decimal_to_snafu(decimal_sum_value)
}

fn part2(_input : String) -> String {
    "1".to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1};


    #[test]
    fn test1() {

        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

        assert_eq!("2=-1=0", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_25.txt");
        assert_eq!("2-=0-=-2=111=220=100", solve(input.to_string(), Part1));
    }
}
