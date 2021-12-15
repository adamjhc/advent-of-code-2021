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
    println!("{}", part_one());
    println!("{}", part_two());
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

fn part_two() -> String {
    let (mut dots, instructions) = parse_input();

    instructions.iter().for_each(|&(fold_direction, value)| {
        dots = dots.iter().fold(HashSet::new(), |mut set, &dot| {
            set.insert(fold(dot, fold_direction, value));
            set
        });
    });

    let (max_x, max_y) = dots.iter().fold((0, 0), |(mut max_x, mut max_y), &(x, y)| {
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }

        (max_x, max_y)
    });

    let output = dots.iter().fold(
        vec![vec!['.'; max_x + 1]; max_y + 1],
        |mut output, &(x, y)| {
            output[y][x] = '#';
            output
        },
    );

    output
        .iter()
        .fold(String::new(), |output, row| {
            output + &row.iter().collect::<String>() + "\n"
        })
        .trim()
        .to_string()
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

fn fold((x, y): (usize, usize), fold_direction: FoldDirection, value: usize) -> (usize, usize) {
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
}

#[cfg(test)]
mod day_13_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_input() {
        assert_eq!(part_one(), 765)
    }

    #[test]
    fn part_two_gives_correct_answer() {
        assert_eq!(
            part_two(),
            "###..####.#..#.####.#....###...##..#..#\n\
             #..#....#.#.#.....#.#....#..#.#..#.#..#\n\
             #..#...#..##.....#..#....#..#.#....####\n\
             ###...#...#.#...#...#....###..#.##.#..#\n\
             #.#..#....#.#..#....#....#....#..#.#..#\n\
             #..#.####.#..#.####.####.#.....###.#..#"
        )
    }
}
