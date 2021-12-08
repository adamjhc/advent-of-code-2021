use utils;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    let mut crab_positions = parse_input();

    crab_positions.sort();

    let median = crab_positions[crab_positions.len() / 2];

    crab_positions.iter().fold(0, |fuel, &pos| {
        fuel + if pos > median {
            pos - median
        } else {
            median - pos
        }
    })
}

fn part_two() -> usize {
    let crab_positions = parse_input();

    let mean = crab_positions.iter().sum::<usize>() / crab_positions.len();

    crab_positions.iter().fold(0, |fuel, &pos| {
        let steps = if pos > mean { pos - mean } else { mean - pos };

        fuel + (steps * (steps + 1) / 2)
    })
}

fn parse_input() -> Vec<usize> {
    utils::read_string("./input.txt")
        .split(",")
        .filter_map(|num| num.parse().ok())
        .collect()
}

#[cfg(test)]
mod day_07_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 344735)
    }

    #[test]
    fn part_two_gives_correct_answer() {
        assert_eq!(part_two(), 96798233)
    }
}
