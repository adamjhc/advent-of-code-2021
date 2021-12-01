use utils;

fn main() {
    println!("{}", part_1());
}

fn part_1() -> u32 {
    let lines = utils::read_lines("./input.txt");

    let mut increases = 0u32;
    let mut previous_depth = u32::MAX;
    for line in lines {
        if let Ok(depth) = line {
            let depth_int = depth.parse().unwrap();
            if depth_int > previous_depth {
                increases += 1;
            }

            previous_depth = depth_int;
        }
    }

    increases
}

#[cfg(test)]
mod day_01_tests {
    use super::*;

    #[test]
    fn part_1_gives_correct_answer() {
        assert_eq!(part_1(), 1233)
    }
}
