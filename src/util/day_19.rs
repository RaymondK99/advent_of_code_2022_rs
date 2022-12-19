use std::cmp::max;
use super::Part;
use regex::Regex;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum MineralType {
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE,
}

#[derive(Debug)]
struct Blueprint {
    number:u32,
    cost_ore_robot:u32,
    cost_clay_robot:u32,
    cost_obsidian_robot_ore:u32,
    cost_obsidian_robot_clay:u32,
    cost_geode_robot_ore:u32,
    cost_geode_robot_obsidian:u32,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    ore_robots:u32,
    ore:u32,
    clay_robots:u32,
    clay:u32,
    obsidian_robots:u32,
    obsidian:u32,
    geode_robots:u32,
    geode:u32,
    minute:u32,

}

impl State {
    fn new() -> State {
        State{ore_robots:1, ore:0, clay:0, clay_robots:0, obsidian_robots:0, obsidian:0, geode_robots:0, geode:0, minute:1}
    }

    fn can_produce_robot(&self, mineral_type:&MineralType) -> bool {
        match mineral_type {
            MineralType::ORE => self.ore_robots > 0,
            MineralType::CLAY => self.ore_robots > 0,
            MineralType::OBSIDIAN => self.clay_robots > 0 && self.ore_robots > 0,
            MineralType::GEODE => self.obsidian_robots > 0 && self.ore_robots > 0,
        }
    }

    fn has_resource_to_produce(&self, mineral_type:&MineralType, blueprint:&Blueprint) -> u32 {
        let (cost_ore, cost_clay, cost_obsidian) = match mineral_type {
            MineralType::ORE => (blueprint.cost_ore_robot as i32,0,0),
            MineralType::CLAY => (blueprint.cost_clay_robot as i32,0,0),
            MineralType::OBSIDIAN => (blueprint.cost_obsidian_robot_ore as i32, blueprint.cost_obsidian_robot_clay as i32, 0),
            MineralType::GEODE => (blueprint.cost_geode_robot_ore as i32, 0, blueprint.cost_geode_robot_obsidian as i32),
        };

        let mut capacity = 0;
        let mut ore = self.ore as i32;
        let mut clay = self.clay as i32;
        let mut obsidian = self.obsidian as i32;
        while cost_ore <= ore && cost_clay <= clay && cost_obsidian <= obsidian {
            capacity +=1;
            ore -= cost_ore;
            clay -= cost_clay;
            obsidian -= cost_obsidian;
        }

        capacity
    }

    fn maxed_out_ore_robots(&self, blueprint:&Blueprint) -> bool {
        let max2 = max(blueprint.cost_obsidian_robot_ore, blueprint.cost_geode_robot_ore);
        let max1 = max(blueprint.cost_ore_robot, blueprint.cost_clay_robot);

        let max_required_ore_rate = max(max1, max2);
        self.ore_robots >= max_required_ore_rate && self.ore >= max_required_ore_rate
    }

    fn max_required_clay_robots(&self, blueprint:&Blueprint) -> bool {
        blueprint.cost_obsidian_robot_clay >= self.clay_robots && self.clay >= blueprint.cost_obsidian_robot_clay
    }

    fn max_required_obsidian_robots(&self, blueprint:&Blueprint) -> bool {
        blueprint.cost_geode_robot_obsidian >= self.obsidian_robots && self.obsidian >= blueprint.cost_geode_robot_obsidian
    }

    fn build_robot(&mut self, mineral_type:&MineralType, blueprint:&Blueprint) {
        match mineral_type {
            MineralType::ORE => {
                self.ore -= blueprint.cost_ore_robot;
                self.ore_robots +=1;
            }
            MineralType::CLAY => {
                self.ore -= blueprint.cost_clay_robot;
                self.clay_robots += 1;
            }
            MineralType::OBSIDIAN => {
                self.ore -= blueprint.cost_obsidian_robot_ore;
                self.clay -= blueprint.cost_obsidian_robot_clay;
                self.obsidian_robots += 1;
            }
            MineralType::GEODE => {
                self.ore -= blueprint.cost_geode_robot_ore;
                self.obsidian -= blueprint.cost_geode_robot_obsidian;
                self.geode_robots += 1;
            }
        };
    }

    fn produce_minerals(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }


    fn try_build_robot(&mut self, mineral_type:&MineralType, blueprint:&Blueprint, rounds:u32) -> bool {
        loop {
            if self.minute == rounds {
                // Final produce
                self.produce_minerals();
                return false;
            } else if self.has_resource_to_produce(mineral_type, blueprint) > 0 {
                // Can build robot
                self.produce_minerals();

                // Build at end of turn
                self.build_robot(mineral_type, blueprint);
                self.minute +=1;
                return true;
            } else {
                // Produce
                self.produce_minerals();
                self.minute += 1;
            }
        }
    }

    fn run(&mut self, blueprint:&Blueprint, rounds:u32) -> u32 {

        let all_types = [MineralType::ORE, MineralType::CLAY, MineralType::OBSIDIAN, MineralType::GEODE];

        // Only produce what we can and need, do not produce robots if we already reached max throughput
        let producible = all_types.iter()
            .filter(|mineral_type| self.can_produce_robot(mineral_type))
            .filter(|&mineral_type| {
                    mineral_type == &MineralType::ORE && !self.maxed_out_ore_robots(blueprint) ||
                    mineral_type == &MineralType::CLAY && !self.max_required_clay_robots(blueprint) ||
                    mineral_type == &MineralType::OBSIDIAN && !self.max_required_obsidian_robots(blueprint) ||
                    mineral_type == &MineralType::GEODE
            }).collect::<Vec<_>>();


        let mut max_geode = vec![];
        for mineral_type in producible.iter() {
            let mut next_state = *self;
            let geode_produced = if next_state.try_build_robot(mineral_type, blueprint, rounds) {
                next_state.run(blueprint, rounds)
            } else {
                next_state.geode
            };

            max_geode.push(geode_produced);
        }

        let max_geodes = max_geode.into_iter().max().unwrap();
        max_geodes
    }

}

impl Blueprint {
    fn parse(line:&str) -> Blueprint  {
        //Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        let re = Regex::new(r"Blueprint (\d*): Each ore robot costs (\d*) ore. Each clay robot costs (\d*) ore. Each obsidian robot costs (\d*) ore and (\d*) clay. Each geode robot costs (\d*) ore and (\d*) obsidian.").unwrap();
        if let Some(captures) =  re.captures(line) {
            let number = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let cost_ore_robot = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let cost_clay_robot = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
            let cost_obsidian_robot_ore = captures.get(4).unwrap().as_str().parse::<u32>().unwrap();
            let cost_obsidian_robot_clay = captures.get(5).unwrap().as_str().parse::<u32>().unwrap();
            let cost_geode_robot_ore = captures.get(6).unwrap().as_str().parse::<u32>().unwrap();
            let cost_geode_robot_obsidian = captures.get(7).unwrap().as_str().parse::<u32>().unwrap();
            Blueprint{number, cost_ore_robot, cost_clay_robot, cost_obsidian_robot_ore, cost_obsidian_robot_clay, cost_geode_robot_ore, cost_geode_robot_obsidian}

        } else {
            panic!("...");
        }
    }
}

fn part1(input : String) -> String {
    let blueprints = input.lines().map(|line| Blueprint::parse(line)).collect::<Vec<_>>();

    let result = blueprints.iter()
        .map(|blueprint|(blueprint.number, State::new().run(blueprint, 24))).collect::<Vec<_>>();

    result.iter().map(|(res, no)| res * no).sum::<u32>().to_string()
}

fn part2(input : String) -> String {
    let blueprints = input.lines().map(|line| Blueprint::parse(line)).collect::<Vec<_>>();
    let first_blueprints = blueprints.chunks(3).next().unwrap();

    let result = first_blueprints.iter()
        .map(|blueprint|(blueprint.number, State::new().run(blueprint, 32))).collect::<Vec<_>>();

    result.iter().map(|(_, res)| *res ).reduce(|a,b| a * b).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const TEST_INPUT:&str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test1() {
        assert_eq!("33", solve(TEST_INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_19.txt");
        assert_eq!("1092", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("3348", solve(TEST_INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_19.txt");

        assert_eq!("3542", solve(input.to_string(), Part2));
    }
}
