use std::collections::VecDeque;
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

fn parse_file_tree(input:String) -> Vec<u32> {
    let mut lines:VecDeque<&str> = input.lines().collect();
    let mut stack:VecDeque<u32> = VecDeque::new();
    let mut dir_sizes = vec![];

    while !lines.is_empty() {
        let cmd = lines.pop_front().unwrap().split(' ').collect::<Vec<_>>();

        if cmd[0].eq("$") && cmd[1].eq("cd") {
            // Change of dir
            let dir_name = cmd[2];
            if dir_name.eq("..") {
                // Pop current dir
                let size = stack.pop_front().unwrap();

                // And add its' size to parent
                *stack.front_mut().unwrap() += size;

                // Add to result
                dir_sizes.push(size);
            } else {
                // Add new sub dir
                stack.push_front(0_u32);
            }
        } else if cmd[0].eq("$") && cmd[1].eq("ls") {
            // List nodes in dir
            while !lines.is_empty() && !lines.front().unwrap().starts_with("$") {
                let node = lines.pop_front().unwrap().split(' ').collect::<Vec<_>>();
                if node[0].ne("dir") {
                    // File -> add size
                    *stack.front_mut().unwrap() += node[0].parse::<u32>().unwrap();
                }
            }
        }
    }

    // Pop current dir and add its' size to parent until we reach the root
    while !stack.is_empty() {
        let size = stack.pop_front().unwrap();

        if !stack.is_empty() {
            *stack.front_mut().unwrap() += size;
        }

        dir_sizes.push(size);
    }

    dir_sizes
}

fn part1(input : String) -> String {
    parse_file_tree(input).iter()
        .filter(|&&size| size <= 100000 )
        .sum::<u32>()
        .to_string()
}


fn part2(input : String) -> String {
    let result = parse_file_tree(input);
    let used_space = *result.last().unwrap();

    result.iter()
        .filter(|&&size| (70_000_000 - used_space + size) >= 30_000_000)
        .min().unwrap().to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!("95437", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_07.txt");

        assert_eq!("1477771", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!("24933642", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_07.txt");

        assert_eq!("3579501", solve(input.to_string(), Part2));
    }
}
