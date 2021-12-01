use utils;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    parse_input().windows(2).filter(|w| w[0] < w[1]).count()
}

fn part_two() -> usize {
    parse_input()
        .windows(4)
        .filter(|w| w[0] + w[1] + w[2] < w[1] + w[2] + w[3])
        .count()
}

fn parse_input() -> Vec<usize> {
    utils::read_string("./input.txt")
        .lines()
        .filter_map(|depth_str| depth_str.parse().ok())
        .collect()
}

#[cfg(test)]
mod day_01_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 1233)
    }

    #[test]
    fn part_two_gives_correct_answer() {
        assert_eq!(part_two(), 1275)
    }
}
