use std::{num::ParseIntError, str::FromStr};

use utils;

struct Command {
    direction: Direction,
    units: usize,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(str: &str) -> Result<Self, <Self as FromStr>::Err> {
        let split: Vec<&str> = str.split(" ").collect();

        Ok(Command {
            direction: split[0].parse()?,
            units: split[1].parse()?,
        })
    }
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ParseIntError;

    fn from_str(str: &str) -> Result<Self, <Self as FromStr>::Err> {
        match str {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => panic!("Error parsing direction"),
        }
    }
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    let (horizontal_position, depth) =
        parse_input()
            .iter()
            .fold(
                (0usize, 0usize),
                |(horizontal_position, depth), command| match command.direction {
                    Direction::Forward => (horizontal_position + command.units, depth),
                    Direction::Down => (horizontal_position, depth + command.units),
                    Direction::Up => (horizontal_position, depth - command.units),
                },
            );

    horizontal_position * depth
}

fn part_two() -> usize {
    let (horizontal_position, depth, _) = parse_input().iter().fold(
        (0usize, 0usize, 0usize),
        |(horizontal_position, depth, aim), command| match command.direction {
            Direction::Forward => (
                horizontal_position + command.units,
                depth + (aim * command.units),
                aim,
            ),
            Direction::Down => (horizontal_position, depth, aim + command.units),
            Direction::Up => (horizontal_position, depth, aim - command.units),
        },
    );

    horizontal_position * depth
}

fn parse_input() -> Vec<Command> {
    utils::read_lines("./input.txt")
        .filter_map(|l| l.unwrap().parse().ok())
        .collect()
}

#[cfg(test)]
mod day_02_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 1524750)
    }

    #[test]
    fn part_two_gives_correct_answer() {
        assert_eq!(part_two(), 1592426537)
    }
}
