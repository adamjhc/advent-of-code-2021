use std::collections::{HashSet, VecDeque};

use itertools::iproduct;
use utils;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> u32 {
    let heightmap = parse_input();

    iproduct!(0..heightmap.len(), 0..heightmap[0].len()).fold(0, |sum_risk_levels, (row, col)| {
        let height = heightmap[row][col];
        let adjacent_locations =
            get_adjacent_locations(row, col, heightmap.len(), heightmap[0].len());

        if adjacent_locations
            .iter()
            .all(|&(row, col)| height < heightmap[row][col])
        {
            sum_risk_levels + height + 1
        } else {
            sum_risk_levels
        }
    })
}

fn part_two() -> u32 {
    let heightmap = parse_input();

    let mut basin_sizes = Vec::new();

    iproduct!(0..heightmap.len(), 0..heightmap[0].len()).for_each(|(row, col)| {
        let height = heightmap[row][col];
        let adjacent_locations =
            get_adjacent_locations(row, col, heightmap.len(), heightmap[row].len());

        if adjacent_locations
            .iter()
            .all(|&(row, col)| height < heightmap[row][col])
        {
            let mut locations_to_check = VecDeque::from(vec![(row, col)]);
            let mut locations_checked = HashSet::new();
            let mut basin_size = 0;

            while let Some((row, col)) = locations_to_check.pop_front() {
                if !locations_checked.contains(&(row, col)) && heightmap[row][col] != 9 {
                    basin_size += 1;
                    get_adjacent_locations(row, col, heightmap.len(), heightmap[row].len())
                        .iter()
                        .for_each(|&location| {
                            if !locations_checked.contains(&location) {
                                locations_to_check.push_back(location)
                            }
                        });
                    locations_checked.insert((row, col));
                }
            }

            basin_sizes.push(basin_size);
        }
    });

    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));

    basin_sizes.iter().take(3).product()
}

fn parse_input() -> Vec<Vec<u32>> {
    utils::read_string("./input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|value| value.to_digit(10))
                .collect()
        })
        .collect()
}

fn get_adjacent_locations(
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
) -> Vec<(usize, usize)> {
    let mut adjacent_locations = Vec::new();

    if row > 0 {
        adjacent_locations.push((row - 1, col));
    }
    if row < max_row - 1 {
        adjacent_locations.push((row + 1, col));
    }
    if col > 0 {
        adjacent_locations.push((row, col - 1));
    }
    if col < max_col - 1 {
        adjacent_locations.push((row, col + 1));
    }

    adjacent_locations
}

#[cfg(test)]
mod day_09_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 512)
    }

    #[test]
    fn part_two_gives_correct_answer() {
        assert_eq!(part_two(), 1600104)
    }
}
