use std::{char::ParseCharError, collections::HashSet, str::FromStr};

use utils;

#[derive(Clone, Copy)]
enum FoldDirection {
    X,
    Y,
}

impl FromStr for FoldDirection {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fold along x" => Ok(FoldDirection::X),
            "fold along y" => Ok(FoldDirection::Y),
            _ => panic!(),
        }
    }
}

fn main() {
    println!("{}", part_one())
}

fn part_one() -> usize {
    let (dots, instructions) = parse_input();

    let (fold_direction, value) = instructions[0];

    let fold = |&(x, y): &(usize, usize)| -> (usize, usize) {
        match fold_direction {
            FoldDirection::X => {
                if x > value {
                    (x - 2 * (x - value), y)
                } else {
                    (x, y)
                }
            }
            FoldDirection::Y => {
                if y > value {
                    (x, y - 2 * (y - value))
                } else {
                    (x, y)
                }
            }
        }
    };

    dots.iter()
        .fold(HashSet::new(), |mut set, dot| {
            set.insert(fold(dot));
            set
        })
        .len()
}

fn parse_input() -> (HashSet<(usize, usize)>, Vec<(FoldDirection, usize)>) {
    let input = utils::read_string("./input.txt");
    let sections: Vec<&str> = input.split("\n\n").collect();

    let dots = sections[0].lines().fold(HashSet::new(), |mut set, dot| {
        let coords: Vec<&str> = dot.split(",").collect();
        set.insert((coords[0].parse().unwrap(), coords[1].parse().unwrap()));
        set
    });

    let instructions = sections[1]
        .lines()
        .fold(Vec::new(), |mut instructions, instruction| {
            let instruction_split: Vec<&str> = instruction.split("=").collect();
            instructions.push((
                instruction_split[0].parse().unwrap(),
                instruction_split[1].parse().unwrap(),
            ));
            instructions
        });

    (dots, instructions)
}

#[cfg(test)]
mod day_13_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_input() {
        assert_eq!(part_one(), 765)
    }
}
