use itertools::iproduct;
use utils;

fn main() {
    println!("{}", part_one())
}

fn part_one() -> u32 {
    let heightmap = parse_input();

    iproduct!(0..heightmap.len(), 0..heightmap[0].len()).fold(0, |sum_risk_levels, (row, col)| {
        let height = heightmap[row][col];
        let mut adjacent_locations = Vec::new();

        if row > 0 {
            adjacent_locations.push(heightmap[row - 1][col]);
        }
        if row < heightmap.len() - 1 {
            adjacent_locations.push(heightmap[row + 1][col]);
        }
        if col > 0 {
            adjacent_locations.push(heightmap[row][col - 1]);
        }
        if col < heightmap[0].len() - 1 {
            adjacent_locations.push(heightmap[row][col + 1]);
        }

        if adjacent_locations.iter().all(|&adjacent| height < adjacent) {
            sum_risk_levels + height + 1
        } else {
            sum_risk_levels
        }
    })
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

#[cfg(test)]
mod day_09_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 512)
    }
}
